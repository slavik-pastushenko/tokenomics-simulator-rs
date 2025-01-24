use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Token.
#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    /// ID for the token.
    pub id: Uuid,

    /// Name of the token.
    pub name: String,

    /// Symbol of the token.
    pub symbol: String,

    /// Total supply of the token.
    pub total_supply: u64,

    /// Number of decimals used for token division.
    pub decimals: u8,

    /// Type of supply.
    pub supply_type: SupplyType,

    /// Annual percentage increase in supply, if supply is inflationary.
    pub inflation_rate: Option<f64>,

    /// Percentage of tokens burned during each transaction, if deflationary.
    pub burn_rate: Option<f64>,

    /// Initial price of the token in simulation
    pub initial_price: f64,
}

/// Type of supply.
#[derive(Debug, Deserialize, Serialize)]
pub enum SupplyType {
    /// Fixed supply.
    #[serde(rename = "fixed")]
    Fixed,

    /// Inflationary supply.
    #[serde(rename = "inflationary")]
    Inflationary,

    /// Deflationary supply.
    #[serde(rename = "deflationary")]
    Deflationary,
}
