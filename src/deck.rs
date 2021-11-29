use std::iter;

use arrayvec::ArrayVec;
use itertools::Itertools;
use rand::{prelude::SliceRandom, Rng};

use crate::cards::{
    self, event,
    list::{all_court_cards, all_event_cards},
    Card,
};

pub struct Deck {
    // The back of the vec is the top of the deck
    cards: Vec<Card>,
}

fn build_deck(rng: &mut impl Rng, player_count: usize) -> Deck {
    let mut court_cards = all_court_cards().collect_vec();
    let mut event_cards = all_event_cards().collect_vec();

    // Shuffle court cards and event cards
    court_cards.shuffle(rng);
    event_cards.shuffle(rng);

    let mut court_cards = court_cards.into_iter();
    let mut event_cards = event_cards.into_iter();

    let mut piles: [Vec<cards::Card>; 6] = brownstone::build(|| Vec::with_capacity(12));
    let pile_size = 5 + player_count;

    // Deal court cards
    piles
        .iter_mut()
        .for_each(|pile| pile.extend(court_cards.by_ref().take(pile_size).map(cards::Card::Court)));

    // Add 2 event cards to the second pile
    piles[5].extend(
        event_cards
            .by_ref()
            .take(2)
            .map(event::Card::Event)
            .map(cards::Card::Event),
    );

    // Add a dominance card and event card to the back 4 piles
    piles[..4].iter_mut().for_each(|pile| {
        pile.extend(
            event_cards
                .by_ref()
                .take(1)
                .map(event::Card::Event)
                .chain(iter::once(event::Card::Dominance))
                .map(cards::Card::Event),
        )
    });

    Deck {
        cards: piles
            .into_iter()
            .map(|mut pile| {
                pile.shuffle(rng);
                pile
            })
            .flatten()
            .collect(),
    }
}
