use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Token.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    /// ID for the token.
    pub id: Uuid,

    /// Name of the token.
    pub name: String,

    /// Symbol of the token.
    pub symbol: Option<String>,

    /// Total supply of the token.
    pub total_supply: u64,

    /// Initial supply of the token, in percentage of total supply.
    pub initial_supply: u64,

    /// Annual percentage increase in supply, if supply is inflationary.
    pub inflation_rate: Option<f64>,

    /// Percentage of tokens burned during each transaction, if deflationary.
    pub burn_rate: Option<f64>,

    /// Initial price of the token in simulation
    pub initial_price: f64,
}

impl Default for Token {
    /// Create a new token with default values.
    ///
    /// # Returns
    ///
    /// New token with default values.
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Token".to_string(),
            symbol: Some("TKN".to_string()),
            total_supply: 1_000_000,
            initial_supply: 100,
            inflation_rate: None,
            burn_rate: None,
            initial_price: 1.0,
        }
    }
}

impl Token {
    /// Calculate the initial supply based on the initial supply percentage.
    ///
    /// # Returns
    ///
    /// Initial supply of the token.
    pub fn initial_supply(&self) -> u64 {
        (self.total_supply as f64 * self.initial_supply as f64 / 100.0).round() as u64
    }
}
