use crate::{map::Region, primitives::Suit};

#[derive(Debug, Clone, Copy)]
pub enum DiscardEvent {
    /// The current climate changes to this
    ChangeSuit(Suit),

    /// All the armies and tribes in this region are destroyed
    Riots(Region),

    /// Ignore all bribes
    DisregardForCustoms,

    /// Used by Public Withdrawal
    NoEffect,

    /// All players must discard 1
    ConfidenceFailure,

    /// Discard all loyalty prizes
    FailureToImpress,

    /// Gifts are worthless
    EmbarrassmentOfRiches,
}

#[derive(Debug, Clone, Copy)]
pub enum PurchaseEvent {
    /// Gifts are worth double
    KohINoorRecovered,

    /// Target player patriots are worthless
    Rumor,

    /// Trade hands with a player
    OtherPersuasiveMethods,

    /// Eat coins. Can't be purchased
    PublicWithdrawal,

    /// May ignore paying bribes
    CourtlyManners,

    /// Build twice as many blocks
    NationBuilding,

    /// Permanently change the climate
    PashtunwaliValues,

    /// Tribes may move and battle
    Nationalism,

    /// Coalitions only need 2 blocks for dominance
    ConflictFatigue,

    /// Your military cards are always considered favored
    NewTactics,

    /// Remove all tribes and armies from a single region
    Rebuke,

    /// Take 3 free rupees
    PersianAristocracy,
}

#[derive(Debug)]
pub struct EventCard {
    pub discard: DiscardEvent,
    pub purchase: PurchaseEvent,
}

#[derive(Debug)]
pub enum Card {
    Event(EventCard),
    Dominance,
}
