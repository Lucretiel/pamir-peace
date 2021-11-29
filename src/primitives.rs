use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Coalition {
    Britain,
    Russia,
    Afghanistan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Suit {
    Political,
    Intelligence,
    Economic,
    Military,
}
