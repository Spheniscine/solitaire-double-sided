use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use crate::game::{Card, DECK_SIZE};

pub const NUM_DEPOTS: usize = 5;

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple, Debug, PartialEq, Eq)]
pub struct BoardPos {
    pub depot_index: usize,
    pub card_index: usize,
}

impl BoardPos {
    pub fn new(depot_index: usize, card_index: usize) -> Self {
        Self { depot_index, card_index }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AnimationAct {
    Move { cards: Vec<Card>, pos1: BoardPos, pos2: BoardPos },
    Flip { card: Card, pos: BoardPos, },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Board {
    pub depots: Vec<Vec<Card>>,
    pub selected: Option<BoardPos>,
    pub animation_acts: Vec<AnimationAct>,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            depots: vec![vec![]; NUM_DEPOTS],
            selected: None,
            animation_acts: vec![],
        }
    }

    pub fn from_deal(deal: &[Card]) -> Self {
        assert_eq!(deal.len(), DECK_SIZE);

        let mut res = Self::empty();
        for (&card, depot) in deal.iter().zip(std::iter::repeat(0..NUM_DEPOTS).flatten()) {
            res.depots[depot].push(card);
        }

        res
    }

    pub fn do_move(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.selected = None;
        let cards = self.depots[pos1.depot_index].drain(pos1.card_index ..).collect();
        self.animation_acts.push(
            AnimationAct::Move { cards, pos1, pos2 }
        );
    }

    pub fn do_flip(&mut self, pos: BoardPos) {
        self.selected = None;
        let card = self.depots[pos.depot_index].pop().unwrap();
        self.animation_acts.push(
            AnimationAct::Flip { card, pos }
        );
    }

    pub fn advance_actions(&mut self) {
        for act in self.animation_acts.drain(..) {
            match act {
                AnimationAct::Move{ cards, pos1: _, pos2 } => {
                    self.depots[pos2.depot_index].extend(cards);
                },
                AnimationAct::Flip { card, pos } => {
                    self.depots[pos.depot_index].push(!card);
                },
            }
        }
    }

    pub fn top_pos(&self, depot: usize) -> BoardPos {
        BoardPos::new(depot, self.depots[depot].len())
    }

    pub fn last_pos(&self, depot: usize) -> BoardPos {
        BoardPos::new(depot, self.depots[depot].len().wrapping_sub(1))
    }
}