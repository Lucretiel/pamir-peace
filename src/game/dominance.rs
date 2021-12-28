use std::{cmp::Reverse, iter};

use enum_map::{enum_map, EnumMap};

use crate::{cards::court::SpecialAbility, player, primitives::Coalition, score};

use super::Game;

struct BlockCount {
    coalition: Coalition,
    count: i8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GameOver {
    pub winner: player::Color,
}

impl Game {
    /// Resolve a dominance check. Run this method *after* discarding any
    /// dominance cards, as it examines the discard pile to determine if this
    /// is the final
    pub fn resolve_dominance_check(&mut self) -> Option<GameOver> {
        // Is this the final dominance check? If so, it scores double, and the
        // game is definitely over
        let dominance_count = self
            .discard
            .iter()
            .filter(|card| card.is_dominance())
            .count();
        let final_dominance_check = dominance_count == 4;

        // Count blocks on the map
        let block_counts = self.map.total_block_counts();

        let mut block_counts: [BlockCount; 3] = brownstone::build_iter(
            block_counts
                .into_iter()
                .map(|(coalition, count)| BlockCount { coalition, count }),
        );

        // Find the coalition with the most blocks
        block_counts.sort_by_key(|entry| Reverse(entry.count));

        // Find how many more blocks they have
        let superiority = block_counts[0].count - block_counts[1].count;

        let required_superiority = match self.effects.conflict_fatigue {
            true => 2,
            false => 4,
        };

        let scores = if superiority >= required_superiority {
            // This coalition is dominant. Score for influence
            let dominant_coalition = block_counts[0].coalition;

            // As part of a successful dominance check, all blocks go home
            self.blocks.add(self.map.clear_blocks());

            // Count up player influence
            let influences = self
                .players
                .iter()
                // Only check players loyal to the dominant coalition
                .filter(|player| player.state.loyalty == dominant_coalition)
                .map(|player| (player.color, player.state.influence(&self.effects)));

            score::compute_scores(
                match final_dominance_check {
                    false => [5, 3, 1],
                    true => [10, 6, 2],
                },
                influences,
            )
        } else {
            // Add up all tribes on the map
            let tribes = self.map.total_tribe_counts();

            // Add up all spies across all courts
            let spies: EnumMap<player::Color, i8> = self
                .players
                .iter()
                .map(|player| &player.state.court)
                .flat_map(|court| &court.cards)
                .fold(EnumMap::default(), |totals, card| {
                    enum_map! {
                        player => totals[player] + card.spies.count(player),
                    }
                });

            // For each player, the number of gifts they've purchased
            let gifts = self
                .players
                .iter()
                .map(|player| (player.color, player.state.gifts.count()));

            let counts =
                gifts.map(|(player, count)| (player, count + tribes[player] + spies[player]));

            score::compute_scores(
                match final_dominance_check {
                    false => [3, 1],
                    true => [6, 2],
                },
                counts,
            )
        };

        // Add scores
        self.players
            .iter_mut()
            .for_each(|player| player.state.score += scores[player.color]);

        // Clear all effects
        self.effects.clear();
        self.players
            .iter_mut()
            .for_each(|player| player.state.effects.clear());

        // Resolve insurrections
        // TODO: if there are no blocks in the supply, the player should take them
        // from elsewhere
        self.players
            .iter()
            .flat_map(|player| {
                player
                    .state
                    .court
                    .cards
                    .iter()
                    .filter(|card| card.ability == Some(SpecialAbility::Insurrection))
                    .map(|card| card.region)
                    .zip(iter::repeat(player.state.loyalty))
            })
            .for_each(|(region, coalition)| {
                let armies = self.blocks.take_up_to(2, coalition);
                self.map.add_armies(region, armies);
            });

        None
    }
}
