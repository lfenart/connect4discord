use std::convert::{TryFrom, TryInto};
use std::fmt;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::emojis;
use connect4::{Game, Mcts, MctsGame, State};

struct DiscordGame {
    game: Game,
    player1: Option<UserId>,
    player2: Option<UserId>,
}

impl DiscordGame {
    fn new(player1: Option<UserId>, player2: Option<UserId>) -> Self {
        Self {
            game: Game::new(),
            player1,
            player2,
        }
    }
}

impl fmt::Display for DiscordGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.game)
    }
}

async fn name(ctx: &Context, user_id: Option<UserId>) -> String {
    if let Some(user_id) = user_id {
        if let Ok(user) = user_id.to_user(&ctx.http).await {
            user.name
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    }
}

#[command]
async fn create(ctx: &Context, msg: &Message) -> CommandResult {
    let mut reply = msg
        .channel_id
        .say(&ctx.http, "```Player 1: \nPlayer 2: ```")
        .await?;
    for reaction in emojis::reactions() {
        reply.react(&ctx.http, reaction).await?;
    }
    let mut player1: Option<UserId> = None;
    let mut player2: Option<UserId> = None;
    let mut game = loop {
        if let Some(reaction) = reply.await_reaction(&ctx).removed(true).await {
            let emoji = &reaction.as_inner_ref().emoji;
            let user = reaction.as_inner_ref().user_id;
            match emojis::Emoji::try_from(emoji) {
                Ok(emojis::Emoji::One) => {
                    if player1.is_none() {
                        player1 = user;
                    } else if player1 == user {
                        player1 = None;
                    }
                    let player1_name = name(ctx, player1).await;
                    let player2_name = name(ctx, player2).await;
                    reply
                        .edit(&ctx.http, |m| {
                            m.content(format!(
                                "```Player 1: {}\nPlayer 2: {}```",
                                player1_name, player2_name
                            ))
                        })
                        .await?;
                }
                Ok(emojis::Emoji::Two) => {
                    if player2.is_none() {
                        player2 = user;
                    } else if player2 == user {
                        player2 = None;
                    }
                    let player1_name = name(ctx, player1).await;
                    let player2_name = name(ctx, player2).await;
                    reply
                        .edit(&ctx.http, |m| {
                            m.content(format!(
                                "```Player 1: {}\nPlayer 2: {}```",
                                player1_name, player2_name
                            ))
                        })
                        .await?;
                }
                Ok(emojis::Emoji::Check) => {
                    break DiscordGame::new(player1, player2);
                }
                _ => (),
            }
        }
    };
    let mut turn = 0;
    let names = (
        if game.player1.is_some() {
            name(ctx, game.player1).await
        } else {
            "X".to_string()
        },
        if game.player2.is_some() {
            name(ctx, game.player2).await
        } else {
            "0".to_string()
        },
    );
    loop {
        let (player, player_name) = if turn == 0 {
            (game.player1, &names.0)
        } else {
            (game.player2, &names.1)
        };
        if let Some(player) = player {
            reply
                .edit(&ctx.http, |m| {
                    m.content(format!(
                        "```Haskell\n{}\n{}, please choose your move (1-7).\n```",
                        game, player_name
                    ))
                })
                .await?;
            if let Some(reaction) = reply
                .await_reaction(&ctx)
                .author_id(player)
                .removed(true)
                .await
            {
                if let Some(emoji) = emojis::Emoji::try_from(&reaction.as_inner_ref().emoji).ok() {
                    if let Some(column) = emoji.try_into().ok() {
                        if game.game.can_play(column) {
                            game.game.play(column);
                            match game.game.state() {
                                State::Win(_) => {
                                    reply
                                        .edit(&ctx.http, |m| {
                                            m.content(format!(
                                                "```Haskell\n{}\n{} wins!\n```",
                                                game, player_name
                                            ))
                                        })
                                        .await?;
                                    break;
                                }
                                State::Draw => {
                                    reply
                                        .edit(&ctx.http, |m| {
                                            m.content(format!("```Haskell\n{}\nDraw.\n```", game))
                                        })
                                        .await?;
                                    break;
                                }
                                State::Unfinished => (),
                            }
                            turn = 2 - turn;
                        }
                    }
                }
            }
        } else {
            reply
                .edit(&ctx.http, |m| {
                    m.content(format!(
                        "```Haskell\n{}\n{} is thinking...\n```",
                        game, player_name
                    ))
                })
                .await?;
            let mut mcts = Mcts::new(game.game.clone());
            mcts.search(500000);
            let (action, _) = mcts.best_action();
            game.game.play(action);
            match game.game.state() {
                State::Win(_) => {
                    reply
                        .edit(&ctx.http, |m| {
                            m.content(format!("```Haskell\n{}\n{} wins!\n```", game, player_name))
                        })
                        .await?;
                    break;
                }
                State::Draw => {
                    reply
                        .edit(&ctx.http, |m| {
                            m.content(format!("```Haskell\n{}\nDraw.\n```", game))
                        })
                        .await?;
                    break;
                }
                State::Unfinished => (),
            }
            turn = 2 - turn;
        }
    }
    Ok(())
}
