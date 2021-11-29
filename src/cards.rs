pub mod court;
pub mod event;
pub mod list;

#[derive(Debug)]
pub enum Card {
    Court(court::Card),
    Event(event::Card),
}
