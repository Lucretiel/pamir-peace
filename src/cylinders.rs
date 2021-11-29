use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use crate::player::{self};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SingleCylinderSet {
    count: i8,
}

impl SingleCylinderSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new bank of cyllinders
    pub fn new_bank() -> Self {
        Self { count: 10 }
    }

    pub fn count(&self) -> i8 {
        self.count
    }

    pub fn add(&mut self, source: Self) {
        self.count += source.count
    }

    pub fn merge(self, rhs: Self) -> Self {
        Self {
            count: self.count + rhs.count,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct CylinderSet {
    bank: EnumMap<player::Color, SingleCylinderSet>,
}

impl CylinderSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self, player: player::Color) -> i8 {
        self.bank[player].count()
    }
}
