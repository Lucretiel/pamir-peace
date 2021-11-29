use std::cmp::Reverse;

use enum_map::{enum_map, EnumMap};
use itertools::Itertools;

use crate::{player, primitives::Coalition, score};

use super::Game;

struct BlockCount {
    coalition: Coalition,
    count: i8,
}

impl Game {
    pub fn resolve_dominance_check(&mut self, doubled: bool) {
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

            // Count up player influence
            let influences = self
                .players
                .iter()
                // Only check players loyal to the dominant coalition
                .filter(|player| player.state.loyalty == dominant_coalition)
                .map(|player| (player.color, player.state.influence(&self.effects)));

            score::compute_block_scores(influences, doubled)
        } else {
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

            let gifts = self
                .players
                .iter()
                .map(|player| (player.color, player.state.gifts.count()));

            let counts =
                gifts.map(|(player, count)| (player, count + tribes[player] + spies[player]));

            score::compute_cylinder_scores(counts, doubled)
        };

        self.players
            .iter_mut()
            .for_each(|player| player.state.score += scores[player.color]);

        // Clear all effects
        self.effects.clear();
        self.players
            .iter_mut()
            .for_each(|player| player.state.effects.clear())
    }
}
