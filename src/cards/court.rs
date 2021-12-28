use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{
    map::Region,
    primitives::{Coalition, Suit},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    One = 1,
    Two = 2,
    Three = 3,
}

impl Rank {
    pub fn value(&self) -> i8 {
        *self as i8
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Impact {
    pub armies: i8,
    pub roads: i8,
    pub spies: i8,
    pub tribes: i8,
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
    pub name: &'static str,
    pub rank: Rank,
    pub suit: Suit,
    pub region: Region,
    pub patriot: Option<Coalition>,
    pub prize: Option<Coalition>,
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

/// Special abilities that a card can have. These are global passives that
/// aren't affected by spies on the card.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpecialAbility {
    /// Cards that share a region are adjacent for spy movement
    StrangeBedfellows,

    /// Don't pay for cards sharing this coalition
    CoalitionInfluence,

    /// This card is always considered favored
    SavvyOperator,

    /// This card is always considered favored
    Irregulars,

    /// Tribes in this region can't be attacked
    Citadel,

    /// Don't pay bribes when playing cards
    CharismaticCourtiers,

    /// Spies killed in battle return to this card
    SafeHouse,

    /// After you take the build action, place one additional block
    Infrastructure,

    /// Your political cards cannot be betrayed
    Bodyguards,

    /// Don't pay for cards sharing this region
    RegionInfluence,

    /// At the start of your turn, place a spy on any card matching this
    /// region
    Blackmail,

    /// Your spies cannot be removed in battles with other spies
    IndispensableAdvisors,

    /// After resolving a dominance check, place two armies in this region
    Insurrection,

    /// Do not pay bribes when taking hostage actions
    CivilServiceReforms,

    /// Tax as though you rule every region
    ClaimOfAncientLineage,

    /// Your armies may move without a road
    IndianSupplies,

    /// Spies move double distance
    WellConnected,
}

#[derive(Debug)]
pub struct Card {
    pub(super) data: &'static CardData,
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
