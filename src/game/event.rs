use enum_map::EnumMap;

use crate::{
    cards::{
        self,
        event::{DiscardEvent, PurchaseEvent},
    },
    player::Color,
    rupees::RupeeSet,
};

use super::Game;

/// When Confidence Failure is resolved, all players must discard a card from
/// their hand. This struct captures all player's choices.
#[derive(Debug, Clone, Copy, Default)]
pub struct DiscardChoices {
    pub choices: EnumMap<Color, Option<usize>>,
}

/// When Confidence Failure is resolved, if any players didn't designate a card
/// to discard, this struct indicates which players must do so.
pub struct MustDiscard {
    /// The set of players who must designate a discard and didn't do so
    pub absent_players: (),
}

impl Game {
    fn apply_discard_event(&mut self, event: DiscardEvent) {
        match event {
            DiscardEvent::ChangeSuit(suit) => self.try_set_climate(suit),
            DiscardEvent::Riots(_) => todo!(),
            DiscardEvent::DisregardForCustoms => self.effects.disregard_for_customs = true,
            DiscardEvent::NoEffect => {}
            DiscardEvent::ConfidenceFailure => todo!(),
            DiscardEvent::FailureToImpress => {
                let discarded_prizes = self
                    .players
                    .iter_mut()
                    .flat_map(|player| player.state.prizes.cards.drain(..))
                    .map(cards::Card::Court);

                self.discard.extend(discarded_prizes)
            }
            DiscardEvent::EmbarrassmentOfRiches => self.effects.embarrassment_of_riches = true,
        }
    }

    fn apply_purchase_event(&mut self, event: PurchaseEvent) {
        let current_player = &mut self.players[self.turn.player];

        match event {
            PurchaseEvent::KohINoorRecovered => current_player.state.effects.kohinoor = true,
            PurchaseEvent::Rumor => todo!(),
            PurchaseEvent::OtherPersuasiveMethods => todo!(),
            PurchaseEvent::PublicWithdrawal => todo!(),
            PurchaseEvent::CourtlyManners => current_player.state.effects.courtly_manners = true,
            PurchaseEvent::NationBuilding => current_player.state.effects.nation_building = true,
            PurchaseEvent::PashtunwaliValues => todo!(),
            PurchaseEvent::Nationalism => current_player.state.effects.nationalism = true,
            PurchaseEvent::ConflictFatigue => self.effects.conflict_fatigue = true,
            PurchaseEvent::NewTactics => current_player.state.effects.new_tactics = true,
            PurchaseEvent::Rebuke => todo!(),
            PurchaseEvent::PersianAristocracy => current_player.state.rupees.add(RupeeSet::new(3)),
        }
    }
}
