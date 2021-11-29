use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use crate::primitives::Coalition;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct BlockSet {
    pub blocks: EnumMap<Coalition, i8>,
}

impl BlockSet {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new_tray() -> Self {
        Self {
            blocks: EnumMap::from_array(brownstone::build_cloned(12)),
        }
    }
}
