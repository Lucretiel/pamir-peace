use std::ops::Deref;

#[derive(Debug)]
pub struct CardData {
    pub discard: (),
    pub buy: (),
}

#[derive(Debug)]
pub struct Card {
    data: &'static CardData,
}

impl Deref for Card {
    type Target = CardData;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl AsRef<CardData> for Card {
    fn as_ref(&self) -> &CardData {
        self.data
    }
}
