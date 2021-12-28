use std::{
    collections::VecDeque,
    mem,
    ops::{Deref, Index, IndexMut},
};

use enum_map::{Enum, EnumMap};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    cards::court::{self},
    cylinders::{CylinderSet, SingleCylinderSet},
    game,
    primitives::{Coalition, Suit},
    rupees::RupeeSet,
};

/// A player color
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Black,
    Grey,
}

/// A card in a player's court
#[derive(Debug)]
pub struct CourtCard {
    /// Details about the card itself
    card: court::Card,

    /// The spies on the card
    pub spies: CylinderSet,

    /// Has the card been used for an action this turn
    pub tapped: bool,
}

impl Deref for CourtCard {
    type Target = court::Card;

    fn deref(&self) -> &court::Card {
        &self.card
    }
}

/// Which of the tableau to play a card
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl CourtCard {
    pub fn new(card: court::Card) -> Self {
        Self {
            card,
            spies: CylinderSet::default(),
            tapped: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct Court {
    pub cards: VecDeque<CourtCard>,
}

impl Court {
    pub fn star_count(&self, suit: Suit) -> i8 {
        self.cards
            .iter()
            .filter(|card| card.suit == suit)
            .map(|card| card.rank.value())
            .sum()
    }
}

#[derive(Debug, Default)]
pub struct Hand {
    pub cards: Vec<court::Card>,
}

/// Permanent modifiers that can affect a player, triggered by event cards.
/// All of these are reset after a Dominance Check.
#[derive(Debug, Default)]
pub struct Effects {
    /// May ignore bribes
    pub courtly_manners: bool,

    /// Gifts are worth an extra influence
    pub kohinoor: bool,

    /// Patriots do not count for influence
    pub rumor: bool,

    /// Your tribes may move and battle
    pub nationalism: bool,

    /// Build twice as many blocks
    pub nation_building: bool,

    /// Military cards are always favored
    pub new_tactics: bool,
}

impl Effects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::new()
    }
}

/// All of the state for a single player
#[derive(Debug)]
pub struct PlayerState {
    /// The player's court cards, in order
    pub court: Court,

    /// The player's hand of cards
    pub hand: Hand,

    /// The player's bank of unused cylinders
    pub bank: SingleCylinderSet,

    /// The player's purchased gifts
    pub gifts: SingleCylinderSet,

    /// The set of cards claimed as prizes
    pub prizes: Hand,

    /// The player's available spending money
    pub rupees: RupeeSet,

    /// They player's coalition
    pub loyalty: Coalition,

    /// Any persistent events cards affecting the player
    pub effects: Effects,

    /// The player's score
    pub score: i8,
}

impl PlayerState {
    /// Create a new player state, suitable for the start of a game. Includes
    /// 10 cylinders and 4 rupees.
    pub fn new(loyalty: Coalition) -> Self {
        Self {
            court: Court::default(),
            hand: Hand::default(),
            bank: SingleCylinderSet::new_bank(),
            gifts: SingleCylinderSet::new(),
            prizes: Hand::default(),
            rupees: RupeeSet::new(4),
            loyalty,
            effects: Effects::default(),
            score: 0,
        }
    }

    /// The number of stars in a given suit
    pub fn star_count(&self, suit: Suit) -> i8 {
        self.court.star_count(suit)
    }

    /// The max court size this player can have, based on the number of
    /// purple stars
    pub fn court_size(&self) -> i8 {
        self.star_count(Suit::Political) + 3
    }

    /// The max hand size this player can have, based on the number of blue
    /// stars
    pub fn hand_size(&self) -> i8 {
        self.star_count(Suit::Intelligence) + 2
    }

    /// Add some cylinders back to the bank
    pub fn discard_cylinders(&mut self, cylinders: SingleCylinderSet) {
        self.bank.add(cylinders)
    }

    /// Find how much influence the player has with their current faction
    pub fn influence(&self, game_effects: &game::Effects) -> i8 {
        let patriots: i8 = match self.effects.rumor {
            false => self
                .hand
                .cards
                .iter()
                .filter(|card| card.patriot.is_some())
                .count()
                .try_into()
                .unwrap(),
            true => 0,
        };

        let gifts = match game_effects.embarrassment_of_riches {
            false => self.gifts.count() * if self.effects.kohinoor { 2 } else { 1 },
            true => 0,
        };

        let prizes: i8 = self.prizes.cards.len().try_into().unwrap();

        patriots + gifts + prizes + 1
    }
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub color: Color,
    pub name: String,
}

impl From<PlayerInit> for Player {
    fn from(init: PlayerInit) -> Self {
        Self {
            name: init.name,
            color: init.color,
            state: PlayerState::new(init.loyalty),
        }
    }
}

#[derive(Debug)]
pub struct PlayerInit {
    pub color: Color,
    pub loyalty: Coalition,
    pub name: String,
}

/// The set of players currently playing a game, in turn order
#[derive(Debug)]
pub struct PlayerSet {
    players: Vec<Player>,
}

#[derive(Debug, Copy, Clone, Error)]
#[error("duplicate player color: {0:?}")]
pub struct DuplicateColor(Color);

impl PlayerSet {
    pub fn new(players: impl IntoIterator<Item = PlayerInit>) -> Result<Self, DuplicateColor> {
        let mut colors: EnumMap<Color, bool> = EnumMap::default();

        players
            .into_iter()
            .map(|init| {
                let color = init.color;
                (!mem::replace(&mut colors[color], true))
                    .then(|| Player::from(init))
                    .ok_or(DuplicateColor(color))
            })
            .try_collect()
            .map(|players| PlayerSet { players })
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut [Player] {
        &mut self.players
    }

    pub fn iter(&self) -> impl Iterator<Item = &Player> {
        self.players.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.players.iter_mut()
    }
}

impl Index<usize> for PlayerSet {
    type Output = Player;

    fn index(&self, index: usize) -> &Self::Output {
        &self.players[index]
    }
}

impl IndexMut<usize> for PlayerSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.players[index]
    }
}

impl Index<Color> for PlayerSet {
    type Output = Player;

    fn index(&self, index: Color) -> &Self::Output {
        self.players
            .iter()
            .find(|player| player.color == index)
            .expect("no player matching color")
    }
}

impl IndexMut<Color> for PlayerSet {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        self.players
            .iter_mut()
            .find(|player| player.color == index)
            .expect("no player matching color")
    }
}
