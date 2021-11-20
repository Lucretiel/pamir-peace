use std::{
    fmt::{self, Debug, Formatter},
    mem,
};

/// A single rupee, can be taken from or added to sets
pub struct Rupee {
    _private: (),
}

impl Debug for Rupee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Rupee")
    }
}

/// A set of rupees
#[derive(Debug, Default)]
pub struct RupeeSet {
    count: u8,
}

impl RupeeSet {
    /// Create a new, empty set of rupees
    pub fn new() -> Self {
        Self { count: 0 }
    }

    /// Get the number of rupees in this set
    pub fn count(&self) -> u8 {
        self.count
    }

    /// Add all the rupees from another bag to this one
    pub fn add(&mut self, from: impl IntoRupeeSet) {
        self.count += from.into_set().count
    }

    /// Take up to count rupees out of the bag
    pub fn take_up_to(&mut self, count: u8) -> RupeeSet {
        if count >= self.count {
            self.take_all()
        } else {
            self.count -= count;
            RupeeSet { count }
        }
    }

    /// Try to take `count` rupees out of the bag. If count is too high,
    /// returns `None`, and `self` is unaffected
    pub fn take_exactly(&mut self, count: u8) -> Option<RupeeSet> {
        let remaining = self.count.checked_sub(count)?;
        self.count = remaining;
        Some(RupeeSet { count })
    }

    /// Try to take a single rupee out of this bag
    pub fn take_one(&mut self) -> Option<Rupee> {
        self.take_exactly(1).map(|_set| Rupee { _private: () })
    }

    /// Take all the rupees out of this bag
    pub fn take_all(&mut self) -> RupeeSet {
        mem::take(self)
    }

    /// Try to dissolve this `RupeeSet`. Returns an error if the set is not empty.
    pub fn done(self) -> Result<(), RupeeSet> {
        (self.count() == 0).then(|| ()).ok_or(self)
    }
}

pub trait IntoRupeeSet {
    fn into_set(self) -> RupeeSet;
}

impl IntoRupeeSet for Rupee {
    fn into_set(self) -> RupeeSet {
        RupeeSet { count: 1 }
    }
}

impl IntoRupeeSet for RupeeSet {
    fn into_set(self) -> RupeeSet {
        self
    }
}

/// An infinite source of rupees. There should only be one of these per game.
#[derive(Debug, Default)]
pub struct RupeeBag {
    _private: (),
}

impl RupeeBag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take_one(&mut self) -> Rupee {
        Rupee { _private: () }
    }

    pub fn take(&mut self, count: u8) -> RupeeSet {
        RupeeSet { count }
    }

    pub fn add(&mut self, rupees: impl IntoRupeeSet) {
        rupees.into_set();
    }
}
