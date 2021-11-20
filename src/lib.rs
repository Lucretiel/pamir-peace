use serde::{Deserialize, Serialize};

pub mod blocks;
pub mod cards;
pub mod market;
pub mod rupees;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Coalition {
    Britain,
    Russia,
    Afghanistan,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerColor {
    Red,
    Blue,
    Yellow,
    Black,
    Grey,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Region {
    Transcaspia,
    Persia,
    Herat,
    Kabul,
    Kandahar,
    Punjab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Suit {
    Political,
    Intelligence,
    Economic,
    Military,
}
