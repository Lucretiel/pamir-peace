use crate::{map::Region, primitives::Suit};

use super::{court, event};

static COURT_CARDS: [court::CardData; 0] = [];

pub fn all_court_cards() -> impl Iterator<Item = court::Card> {
    COURT_CARDS.iter().map(|card| court::Card { data: card })
}

pub fn all_event_cards() -> impl Iterator<Item = event::EventCard> {
    use event::{DiscardEvent::*, PurchaseEvent::*};

    [
        (NoEffect, PublicWithdrawal),
        (ConfidenceFailure, OtherPersuasiveMethods),
        (ChangeSuit(Suit::Political), Rebuke),
        (Riots(Region::Persia), PersianAristocracy),
        (DisregardForCustoms, CourtlyManners),
        (EmbarrassmentOfRiches, KohINoorRecovered),
        (FailureToImpress, Rumor),
        (ChangeSuit(Suit::Intelligence), PashtunwaliValues),
        (Riots(Region::Herat), Nationalism),
        (Riots(Region::Punjab), ConflictFatigue),
        (Riots(Region::Kabul), NationBuilding),
        (ChangeSuit(Suit::Military), NewTactics),
    ]
    .into_iter()
    .map(|(discard, purchase)| event::EventCard { purchase, discard })
}
