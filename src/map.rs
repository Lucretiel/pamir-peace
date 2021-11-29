use std::{cmp, collections::HashMap, iter};

use enum_map::{enum_map, Enum, EnumMap};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{blocks::BlockSet, cylinders::CylinderSet, player, primitives::Coalition};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter, Enum,
)]
pub enum Region {
    Transcaspia,
    Persia,
    Herat,
    Kabul,
    Kandahar,
    Punjab,
}

pub use Region::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Border {
    front: Region,
    back: Region,
}

impl Serialize for Border {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.front, self.back).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Border {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|(front, back)| Self::new(front, back))
    }
}

impl Border {
    pub fn new(front: Region, back: Region) -> Self {
        assert!(front != back);
        let (front, back) = (cmp::min(front, back), cmp::max(front, back));
        Self { front, back }
    }

    pub fn front(&self) -> Region {
        self.front
    }

    pub fn back(&self) -> Region {
        self.back
    }

    pub fn borders(&self, region: Region) -> bool {
        self.front == region || self.back == region
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionOccupants {
    pub armies: BlockSet,
    pub tribes: CylinderSet,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BorderOccupants {
    pub roads: BlockSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    regions: EnumMap<Region, RegionOccupants>,
    borders: HashMap<Border, BorderOccupants>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            regions: EnumMap::from_array(Default::default()),

            borders: [
                (Transcaspia, Persia),
                (Transcaspia, Herat),
                (Transcaspia, Kabul),
                (Persia, Herat),
                (Herat, Kabul),
                (Herat, Kandahar),
                (Kabul, Kandahar),
                (Kabul, Punjab),
                (Kandahar, Punjab),
            ]
            .into_iter()
            .map(|(front, back)| Border::new(front, back))
            .zip(iter::repeat_with(BorderOccupants::default))
            .collect(),
        }
    }

    /// Count all the blocks on the map
    pub fn total_block_counts(&self) -> EnumMap<Coalition, i8> {
        let armies = self.regions.values().map(|region| &region.armies);
        let roads = self.borders.values().map(|border| &border.roads);

        armies
            .chain(roads)
            .fold(EnumMap::default(), |totals, blockset| {
                enum_map! { coalition => totals[coalition] + blockset.blocks[coalition] }
            })
    }

    /// Count all the tribes on the map
    pub fn total_tribe_counts(&self) -> EnumMap<player::Color, i8> {
        self.regions.values().map(|region| &region.tribes).fold(
            EnumMap::default(),
            |counts, tribes| {
                enum_map! {
                    player => counts[player] + tribes.count(player)
                }
            },
        )
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
