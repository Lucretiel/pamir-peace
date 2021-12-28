use std::collections::HashSet;

use super::Game;

/// The set of cards the player will discard at the end of their turn
#[derive(Debug, Clone)]
pub struct Discards {
    pub court: HashSet<usize>,
    pub hand: HashSet<usize>,
}



impl Game {
    pub fn end_turn(&mut self, discards: Discards)
}
