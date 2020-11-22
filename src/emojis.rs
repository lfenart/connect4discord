use std::convert::{TryFrom, TryInto};

use serenity::model::prelude::*;

pub enum Emoji {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Check,
}

impl TryInto<u8> for Emoji {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Emoji::One => Ok(0),
            Emoji::Two => Ok(1),
            Emoji::Three => Ok(2),
            Emoji::Four => Ok(3),
            Emoji::Five => Ok(4),
            Emoji::Six => Ok(5),
            Emoji::Seven => Ok(6),
            Emoji::Check => Err(()),
        }
    }
}

impl Into<ReactionType> for Emoji {
    fn into(self) -> ReactionType {
        match self {
            Emoji::One => ReactionType::Unicode("1️⃣".into()),
            Emoji::Two => ReactionType::Unicode("2️⃣".into()),
            Emoji::Three => ReactionType::Unicode("3️⃣".into()),
            Emoji::Four => ReactionType::Unicode("4️⃣".into()),
            Emoji::Five => ReactionType::Unicode("5️⃣".into()),
            Emoji::Six => ReactionType::Unicode("6️⃣".into()),
            Emoji::Seven => ReactionType::Unicode("7️⃣".into()),
            Emoji::Check => ReactionType::Unicode("✅".into()),
        }
    }
}

impl TryFrom<&ReactionType> for Emoji {
    type Error = ();

    fn try_from(reaction_type: &ReactionType) -> Result<Self, Self::Error> {
        match reaction_type.as_data().as_str() {
            "1️⃣" => Ok(Emoji::One),
            "2️⃣" => Ok(Emoji::Two),
            "3️⃣" => Ok(Emoji::Three),
            "4️⃣" => Ok(Emoji::Four),
            "5️⃣" => Ok(Emoji::Five),
            "6️⃣" => Ok(Emoji::Six),
            "7️⃣" => Ok(Emoji::Seven),
            "✅" => Ok(Emoji::Check),
            _ => Err(()),
        }
    }
}

pub fn reactions() -> Vec<Emoji> {
    vec![
        Emoji::One,
        Emoji::Two,
        Emoji::Three,
        Emoji::Four,
        Emoji::Five,
        Emoji::Six,
        Emoji::Seven,
        Emoji::Check,
    ]
}
