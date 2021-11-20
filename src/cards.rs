pub mod court;
pub mod event;

#[derive(Debug)]
pub enum Card {
    Court(court::Card),
    Event(event::Card),
    Dominance,
}
