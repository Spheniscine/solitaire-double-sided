use std::time::Duration;

use rand::{Rng, RngExt, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::{components::LocalStorage, game::{Board, BoardPos, Card, DECK_SIZE, NUM_RANKS, RANKS, SettingsState}};

impl Board {
    pub fn can_flip(&self, pos: BoardPos) -> bool {
        !self.depots[pos.depot_index].is_empty() && pos == self.last_pos(pos.depot_index)
    }
}

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ActionRecord {
    Move { pos1: BoardPos, pos2: BoardPos },
    Flip { pos: BoardPos },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        
        for rank in RANKS {
            for back_rank in RANKS {
                let mut card = Card { rank, back_rank, is_white: false };
                if rng.random() {
                    card = !card;
                }
                deck.push(card);
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
        };

        res.new_game();
        res
    }

    pub fn new_game(&mut self) {
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;

        loop {
            let deal = Self::new_deal(&mut rand::rng());
            self.board = Board::from_deal(&deal);
            self.deal = deal;

            if !self.is_won() { break; }
        }
        
        LocalStorage.save_game_state(&self);
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn is_won(&self) -> bool {
        self.board.depots.iter().all(|depot| {
            depot.len() == NUM_RANKS && self.is_stack(&depot)
        })
    }

    pub fn is_over(&self) -> bool {
        self.is_won()
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        back.is_white != front.is_white && front.rank + 1 == back.rank
    }

    pub fn is_stack(&self, slice: &[Card]) -> bool {
        slice.windows(2).all(|w| self.can_stack(w[0], w[1]))
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.board.do_move(pos1, pos2);
        self.history.push(ActionRecord::Move { pos1, pos2 })
    }

    fn do_flip_raw(&mut self, pos: BoardPos) {
        self.board.do_flip(pos);
        self.history.push(ActionRecord::Flip { pos })
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn can_select(&mut self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        self.is_stack(&slice)
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_over() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            let dest = BoardPos::new(pos.depot_index, pos.card_index.wrapping_add(1));
            if !self.can_move(src, dest) { return; }
            self.undo_stack.push(self.history.len());
            self.do_move_raw(src, dest);
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    pub fn can_move(&self, pos1: BoardPos, pos2: BoardPos) -> bool {
        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        if pos2.card_index != depot2.len() { return false; }

        let card = depot1[pos1.card_index];

        depot2.last().is_none_or(|&c| self.can_stack(c, card))
    }

    pub fn can_flip(&self, pos: BoardPos) -> bool {
        self.board.can_flip(pos)
    }

    fn flip_intent(&mut self, pos: BoardPos) {
        if !self.can_flip(pos) { return; }
        self.undo_stack.push(self.history.len());
        self.do_flip_raw(pos);
    }

    pub fn oncontextmenu(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_over() { return; }

        self.flip_intent(pos);
    }

    pub fn onclick_flip(&mut self) {
        if self.is_busy() { return; }
        if self.is_over() { return; }

        let Some(pos) = self.board.selected else {return};
        self.flip_intent(pos);
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    pub fn undo(&mut self) {
        if self.is_busy() || !self.undo_possible() { return; }
        let Some(target_len) = self.undo_stack.pop() else {return};
        while self.history.len() > target_len {
            let rec = self.history.pop().unwrap();
            match rec {
                ActionRecord::Move { pos1, pos2 } => {
                    self.board.do_move(pos2, pos1);
                },
                ActionRecord::Flip { pos } => {
                    self.board.do_flip(pos);
                },
            }
            
            self.board.advance_actions(); // no animation, as repeated card moves on same card causes problems
        }
        LocalStorage.save_game_state(&self);
    }

    pub fn restart(&mut self) {
        if self.history.is_empty() || !self.undo_possible() { return; }
        self.board = Board::from_deal(&self.deal);
        self.history.clear();
        self.undo_stack.clear();

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn new_settings_state(&self) -> SettingsState {
        SettingsState {
            allow_undo: self.allow_undo,
        }
    }

    pub fn apply_settings(&mut self, settings: &SettingsState){
        self.allow_undo = settings.allow_undo;
        LocalStorage.save_game_state(&self);
    }
}