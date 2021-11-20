use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{Coalition, Region, Suit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    One,
    Two,
    Three,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Impact {
    pub armies: u8,
    pub roads: u8,
    pub spies: u8,
    pub tribes: u8,
    pub leverage: bool,
    pub favor: Option<Suit>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ActionSet {
    pub tax: bool,
    pub gift: bool,
    pub build: bool,
    pub movement: bool,
    pub betray: bool,
    pub battle: bool,
}

impl ActionSet {
    pub fn count(&self) -> usize {
        [
            self.tax,
            self.gift,
            self.build,
            self.movement,
            self.betray,
            self.battle,
        ]
        .into_iter()
        .filter(|&b| b)
        .count()
    }

    pub fn has(&self, action: CardAction) -> bool {
        match action {
            CardAction::Tax => self.tax,
            CardAction::Gift => self.gift,
            CardAction::Build => self.build,
            CardAction::Move => self.movement,
            CardAction::Betray => self.betray,
            CardAction::Battle => self.battle,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardData {
    pub id: u8,
    pub name: &'static str,
    pub rank: Rank,
    pub suit: Suit,
    pub region: Region,
    pub patriot: Option<Coalition>,
    pub price: Option<Coalition>,
    pub impact: Impact,
    pub actions: ActionSet,
    pub ability: Option<SpecialAbility>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CardAction {
    Tax,
    Gift,
    Build,
    Move,
    Betray,
    Battle,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpecialAbility {}

#[derive(Debug)]
pub struct Card {
    data: &'static CardData,
}

impl Deref for Card {
    type Target = CardData;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl AsRef<CardData> for Card {
    fn as_ref(&self) -> &CardData {
        self.data
    }
}
