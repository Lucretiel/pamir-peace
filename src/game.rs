mod dominance;
mod end_turn;
mod event;

use enum_map::EnumMap;

use crate::{
    cards::Card,
    map::Map,
    player::{self, PlayerSet, PlayerState},
    primitives::Suit,
};

/// Permanent game effects (triggered by event cards). All of these are
/// reset by a Dominance Check

#[derive(Debug, Default)]
pub struct Effects {
    /// Coalitions only require 2 blocks for dominance
    pub conflict_fatigue: bool,

    /// The climate cannot be changed
    pub pashtunwali_values: bool,

    /// Gifts are not worth influence
    pub embarrassment_of_riches: bool,

    /// Ignore all bribes
    pub disregard_for_customs: bool,
}

impl Effects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::new()
    }
}

/// The state of the current player's turn
#[derive(Debug)]
pub struct TurnState {
    pub player: usize,
    pub actions_taken: i8,
}

#[derive(Debug)]
pub struct Game {
    pub map: Map,
    pub climate: Suit,
    pub effects: Effects,
    pub players: PlayerSet,
    pub discard: Vec<Card>,
}

impl Game {
    /// Attempt to change the current climate. Fails if Pashtunwali Values is
    /// in effect
    fn try_set_climate(&mut self, climate: Suit) {
        if !self.effects.pashtunwali_values {
            self.climate = climate
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub name: String,
    pub color: player::Color,
}
