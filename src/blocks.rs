use std::{iter::Sum, mem};

use enum_map::{enum_map, EnumMap};
use serde::{Deserialize, Serialize};

use crate::primitives::Coalition;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct BlockSet {
    blocks: EnumMap<Coalition, i8>,
}

impl BlockSet {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new_tray() -> Self {
        Self {
            blocks: enum_map! {_ => 12},
        }
    }

    pub fn add(&mut self, source: BlockSet) {
        self.blocks = enum_map! {
            coalition => self.blocks[coalition] + source.blocks[coalition],
        };
    }

    /// Remove all blocks from this blockset and return them
    pub fn take_all(&mut self) -> BlockSet {
        mem::take(self)
    }

    /// Take up to `count` blocks of the given coalition and return them
    pub fn take_up_to(&mut self, count: i8, coalition: Coalition) -> BlockSet {
        let count = if count > self.blocks[coalition] {
            mem::replace(&mut self.blocks[coalition], 0)
        } else {
            self.blocks[coalition] -= count;
            count
        };

        Self {
            blocks: enum_map! {
                c => if c == coalition { count } else { 0 }
            },
        }
    }

    pub fn count(&self, coalition: Coalition) -> i8 {
        self.blocks[coalition]
    }
}

impl Sum<BlockSet> for BlockSet {
    fn sum<I: Iterator<Item = BlockSet>>(iter: I) -> Self {
        iter.fold(Self::empty(), |totals, blocks| Self {
            blocks: enum_map! {
                coalition => totals.blocks[coalition] + blocks.blocks[coalition]
            },
        })
    }
}
