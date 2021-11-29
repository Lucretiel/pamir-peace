use std::mem;

/// A single rupee, can be taken from or added to sets
#[derive(Debug, Clone, Copy, Default)]
pub struct Rupee;

/// A set of rupees
#[derive(Debug, Clone, Copy, Default)]
pub struct RupeeSet {
    count: i8,
}

impl RupeeSet {
    /// Create a new, empty set of rupees
    pub fn empty() -> Self {
        Self::new(0)
    }

    /// Create a new set of rupees.
    pub fn new(count: i8) -> Self {
        Self { count }
    }

    /// Get the number of rupees in this set
    pub fn count(&self) -> i8 {
        self.count
    }

    /// Add all the rupees from another bag to this one
    pub fn add(&mut self, from: impl IntoRupeeSet) {
        self.count += from.into_set().count
    }

    /// Take up to count rupees out of the bag
    pub fn take_up_to(&mut self, count: i8) -> RupeeSet {
        if count >= self.count {
            self.take_all()
        } else {
            self.count -= count;
            RupeeSet { count }
        }
    }

    /// Try to take `count` rupees out of the bag. If count is too high,
    /// returns `None`, and `self` is unaffected
    pub fn take_exactly(&mut self, count: i8) -> Option<RupeeSet> {
        if count > self.count {
            None
        } else {
            self.count -= count;
            Some(RupeeSet { count })
        }
    }

    /// Try to take a single rupee out of this bag
    pub fn take_one(&mut self) -> Option<Rupee> {
        self.take_exactly(1).map(|_set| Rupee)
    }

    /// Take all the rupees out of this bag
    pub fn take_all(&mut self) -> RupeeSet {
        mem::take(self)
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
