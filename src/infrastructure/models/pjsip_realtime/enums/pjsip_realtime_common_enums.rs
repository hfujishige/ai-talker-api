use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "turn_on_off", rename_all = "lowercase")]
pub enum TurnOnOff {
    #[serde(alias = "0")]
    Zero,
    #[serde(alias = "1")]
    One,
    Off,
    On,
    False,
    True,
    No,
    Yes,
}

impl fmt::Display for TurnOnOff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &'static str = match self {
            TurnOnOff::Zero => "0",
            TurnOnOff::One => "1",
            TurnOnOff::Off => "off",
            TurnOnOff::On => "on",
            TurnOnOff::False => "false",
            TurnOnOff::True => "true",
            TurnOnOff::No => "no",
            TurnOnOff::Yes => "yes",
        };
        write!(f, "{}", s)
    }
}
