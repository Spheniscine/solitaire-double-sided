use std::ops::{Not, RangeInclusive};

use serde::{Deserialize, Serialize, de::Visitor};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub rank: u8,
    pub back_rank: u8,
    pub is_white: bool,
}

pub const RANK_MIN: u8 = 1;
pub const RANK_MAX: u8 = 5;
pub const RANKS: RangeInclusive<u8> = RANK_MIN ..= RANK_MAX;
pub const NUM_RANKS: usize = (RANK_MAX - RANK_MIN) as usize + 1;
pub const DECK_SIZE: usize = NUM_RANKS * NUM_RANKS;

impl Not for Card {
    type Output = Self;

    fn not(self) -> Self::Output {
        Card {
            rank: self.back_rank,
            back_rank: self.rank,
            is_white: !self.is_white
        }
    }
}

impl Card {
    pub fn code(self) -> String {
        format!("{}{}{}", self.rank, if self.is_white {'W'} else {'B'}, self.back_rank)
    }
    pub fn from_code(code: &str) -> Option<Self> {
        let mut it = code.chars();
        let rank = it.next()? as u8 - b'0';
        let is_white = it.next()? == 'W';
        let back_rank = it.next()? as u8 - b'0';
        Some(Card { rank, back_rank, is_white })
    }
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.code())
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MyVisitor;
        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Card;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "card code")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Card::from_code(v).ok_or_else(|| E::custom(format!("invalid card code: {}", v)))
            }
        }
        deserializer.deserialize_str(MyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Card;

    #[test]
    fn card_to_code_test() {
        let card = Card {
            rank: 5, back_rank: 1, is_white: false
        };
        assert_eq!("5B1", card.code())
    }

    #[test]
    fn card_from_code_test() {
        let card = Card {
            rank: 5, back_rank: 1, is_white: false
        };
        assert_eq!(Some(card), Card::from_code("5B1"))
    }
}