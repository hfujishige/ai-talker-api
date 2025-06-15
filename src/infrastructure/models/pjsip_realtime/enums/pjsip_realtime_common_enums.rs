use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "lowercase")]
pub enum TurnOnOff {
    #[serde(alias = "0")] Zero,
    #[serde(alias = "1")] One,
    Off,
    On,
    False,
    True,
    No,
    Yes,
}
