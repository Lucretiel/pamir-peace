mod dominance;
mod end_turn;
mod event;

use crate::{
    blocks::BlockSet,
    cards::Card,
    map::Map,
    market::Market,
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

/// The request queue is the set of player decisions that need to be played
/// before normal play can continue.
#[derive(Debug)]
pub struct RequestQueue {}

#[derive(Debug)]
pub struct Game {
    /// The actual map of afghanistan
    pub map: Map,

    /// The market of cards available for purchase
    pub market: Market,

    /// The current favored suit
    pub climate: Suit,

    /// The current set of ongoing effects from event cards
    pub effects: Effects,

    /// The bank of blocks that are not currently on the map
    pub blocks: BlockSet,

    /// The players in the game along with all their state (court cards, hands,
    /// etc)
    pub players: PlayerSet,

    /// The discard pile of cards
    pub discard: Vec<Card>,

    /// Information about the current turn- whose turn it is, how many actions
    /// they have, etc
    pub turn: TurnState,
}

impl Game {
    /// Attempt to change the current climate. Fails if Pashtunwali Values is
    /// in effect
    pub fn try_set_climate(&mut self, climate: Suit) {
        if !self.effects.pashtunwali_values {
            self.climate = climate
        }
    }
}
