use std::{
    cmp::Reverse,
    iter::{self},
};

use arrayvec::ArrayVec;
use enum_map::EnumMap;
use itertools::Itertools;

use crate::{
    player,
    util::{EnumSet, Sum},
};

/// Compute the scores for a scoring event.
pub fn compute_scores(
    // The score buckets for each player. For instance, on a successful
    // dominance check, these are [5, 3, 1]
    buckets: impl IntoIterator<Item = i8>,
    tallies: impl IntoIterator<Item = (player::Color, i8)>,
) -> EnumMap<player::Color, i8> {
    let mut tallies: ArrayVec<(player::Color, i8), 5> = tallies.into_iter().collect();

    tallies.sort_by_key(|&(_, tally)| Reverse(tally));

    tallies
        .into_iter()
        .zip(buckets.into_iter().chain(iter::repeat(0)))
        .group_by(|((_, tally), _)| *tally)
        .into_iter()
        .flat_map(|(_, group)| {
            // Each group is defined by the set of players with the same tally
            // These players evenly split all points, rounding down

            // Get all the players who are scoring and how many points they're
            // getting
            let (players, Sum { sum: total_points }): (EnumSet<player::Color>, Sum<i8>) =
                group.map(|((player, _), points)| (player, points)).unzip();

            // Divide the points among all players
            let points_per_player = total_points / players.len() as i8;

            // Emit pairs of (player, points)
            players
                .into_iter()
                .map(move |player| (player, points_per_player))
        })
        // Merge all the player scores together
        .fold(EnumMap::default(), |mut scores, (player, score)| {
            scores[player] += score;
            scores
        })
}

pub fn compute_block_scores(
    influence_tallies: impl IntoIterator<Item = (player::Color, i8)>,
    doubled: bool,
) -> EnumMap<player::Color, i8> {
    compute_scores(
        if doubled { [10, 6, 2] } else { [5, 3, 1] },
        influence_tallies,
    )
}

pub fn compute_cylinder_scores(
    cylinder_tallies: impl IntoIterator<Item = (player::Color, i8)>,
    doubled: bool,
) -> EnumMap<player::Color, i8> {
    compute_scores(if doubled { [6, 2] } else { [3, 1] }, cylinder_tallies)
}

#[cfg(test)]
mod tests {
    use super::*;

    use enum_map::enum_map;

    use player::Color::*;

    macro_rules! test_case {
        (
            $buckets:expr;
            $($color:ident : $tally:expr => $score:expr,)*
        ) => {
            {
                let scores = compute_scores($buckets, [$(($color, $tally)),*]);

                assert_eq!(scores, enum_map! {
                    $($color => $score,)*
                    _ => 0,
                });
            }
        };
    }

    macro_rules! test_cases {
        ($(
            $buckets:expr => {$(
                $test_name:ident:
                $($color:ident : $tally:expr => $score:expr),* ;
            )*}
        )*) => {
            $(
                $(
                    #[test]
                    fn $test_name() {
                        test_case! {
                            $buckets;
                            $($color : $tally => $score,)*
                        }
                    }
                )*
        )*
        }
    }

    test_cases! {
        [5, 3, 1] => {
            test_basic:
                Red: 3 => 5,
                Blue: 2 => 3,
                Yellow: 1 => 1;

            test_two:
                Red: 2 => 5,
                Yellow: 1 => 3;

            test_tie_3:
                Red: 1 => 3,
                Blue: 1 => 3,
                Yellow: 1 => 3;

            test_tie_2:
                Red: 2 => 4,
                Blue: 2 => 4,
                Yellow: 1 => 1;

            test_tie_lower_2:
                Red: 3 => 5,
                Blue: 1 => 2,
                Yellow: 1 => 2;

            test_tie_only_2:
                Red: 2 => 4,
                Blue: 2 => 4;

            test_lower_tie:
                Red: 3 => 5,
                Blue: 2 => 1,
                Yellow: 2 => 1,
                Black: 2 => 1;
        }
        [3, 1] => {
            test_basic_2:
                Blue: 2 => 3,
                Yellow: 1 => 1;

            test_tie:
                Blue: 2 => 2,
                Yellow: 2 => 2;

            test_low_tie:
                Blue: 2 => 3,
                Yellow: 1 => 0,
                Red: 1 => 0;

            test_zeroes:
                Blue: 1 => 3,
                Yellow: 0 => 1;
        }
    }
}
