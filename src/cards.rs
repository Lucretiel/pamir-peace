pub mod court;
pub mod event;
pub mod list;

#[derive(Debug)]
pub enum Card {
    Court(court::Card),
    Event(event::Card),
}

impl Card {
    pub fn is_dominance(&self) -> bool {
        matches!(self, Card::Event(event::Card::Dominance))
    }
}
