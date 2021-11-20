use std::mem;

use itertools::{Itertools, Position};

use crate::{
    cards::Card,
    rupees::{IntoRupeeSet, RupeeSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Column {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl Column {
    pub fn cost(&self) -> u8 {
        *self as u8
    }

    fn index(&self) -> usize {
        *self as usize
    }
}

/// A card in the market, which has both the card itself as well as a pile of
/// rupees
#[derive(Debug)]
pub struct MarketCard {
    card: Card,
    rupees: RupeeSet,

    /// If true, a rupee was placed on this card this turn and this card is
    /// ineligible to be bought
    mark: bool,
}

impl MarketCard {
    pub fn new(card: Card) -> Self {
        Self {
            card,
            rupees: RupeeSet::new(),
            mark: false,
        }
    }

    pub fn card(&self) -> &Card {
        &self.card
    }

    pub fn into_parts(self) -> (Card, RupeeSet) {
        (self.card, self.rupees)
    }

    pub fn add_rupees(&mut self, rupees: impl IntoRupeeSet) {
        let rupees = rupees.into_set();

        if rupees.count() > 0 {
            self.mark = true;
            self.rupees.add(rupees);
        }
    }
}

#[derive(Debug)]
pub struct MarketRow {
    cards: [Option<MarketCard>; 6],
}

impl MarketRow {
    /// Get a view of this row of cards
    pub fn view(&self) -> &[Option<MarketCard>; 6] {
        &self.cards
    }

    pub fn get_card(&self, column: Column) -> Option<&MarketCard> {
        self.cards.get(column.index())?.as_ref()
    }

    pub fn count(&self) -> usize {
        self.cards.iter().filter(|card| card.is_some()).count()
    }

    /// Shift all cards to the left, then fill the remaining slots with cards
    /// from some source, until the row is full or the source runs out
    pub fn fill_from(&mut self, cards: &mut impl Iterator<Item = Card>) {
        let existing_cards = mem::take(&mut self.cards)
            .into_iter()
            .filter_map(|slot| slot);

        let new_cards = cards.map(MarketCard::new);
        let cards = existing_cards.chain(new_cards);

        cards.zip(&mut self.cards).for_each(|(card, slot)| {
            debug_assert!(slot.is_none());
            *slot = Some(card);
        });
    }

    /// Attempt to pay the cost of an ability. Rupees are placed one at a time
    /// from back to front. Any excess rupees are all piled onto the first card.
    /// Returns an error, returning the set, if there are *no* cards in this row.
    pub fn spend_for_ability(&mut self, mut payment: RupeeSet) -> Result<(), RupeeSet> {
        for slot in self
            .cards
            .iter_mut()
            .rev()
            .filter_map(|slot| slot.as_mut())
            .with_position()
        {
            match slot {
                Position::Last(card) | Position::Only(card) => card.rupees.add(payment.take_all()),
                Position::First(card) | Position::Middle(card) => match payment.take_one() {
                    None => return Ok(()),
                    Some(rupee) => card.rupees.add(rupee),
                },
            }
        }

        payment.done()
    }

    /// Take a card without spending any money
    pub fn take_card(&mut self, column: Column) -> Option<MarketCard> {
        self.cards[column.index()].take()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Row {
    Top = 0,
    Bottom = 1,
}

impl Row {
    pub fn other(self) -> Row {
        match self {
            Row::Top => Row::Bottom,
            Row::Bottom => Row::Top,
        }
    }
}

#[derive(Debug)]
pub struct Market {
    top: MarketRow,
    bottom: MarketRow,
}

impl Market {
    pub fn get_row(&self, row: Row) -> &MarketRow {
        match row {
            Row::Top => &self.top,
            Row::Bottom => &self.bottom,
        }
    }

    pub fn fill_from(&mut self, cards: &mut impl Iterator<Item = Card>) {
        self.top.fill_from(cards);
        self.bottom.fill_from(cards);
    }
}
