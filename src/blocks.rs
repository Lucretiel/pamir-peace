use crate::Coalition;

#[derive(Debug)]
pub struct BlockSet {
    pub british: i8,
    pub russian: i8,
    pub afghan: i8,
}

impl BlockSet {
    pub fn empty() -> Self {
        Self {
            british: 0,
            russian: 0,
            afghan: 0,
        }
    }

    pub fn new_tray() -> Self {
        Self {
            british: 12,
            russian: 12,
            afghan: 12,
        }
    }

    pub fn get(&self, coalition: Coalition) -> i8 {
        match coalition {
            Coalition::Britain => self.british,
            Coalition::Russia => self.russian,
            Coalition::Afghanistan => self.afghan,
        }
    }

    pub fn get_mut(&mut self, coalition: Coalition) -> &mut i8 {
        match coalition {
            Coalition::Britain => &mut self.british,
            Coalition::Russia => &mut self.russian,
            Coalition::Afghanistan => &mut self.afghan,
        }
    }
}
