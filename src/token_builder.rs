use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{SimulationError, Token, UnlockEvent};

/// Builder for creating a new token.
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct TokenBuilder {
    /// Name of the token.
    name: Option<String>,

    /// Symbol of the token.
    symbol: Option<String>,

    /// Total supply of the token.
    total_supply: Option<i64>,

    /// Current supply of the token.
    current_supply: Option<f64>,

    /// Initial supply of the token, in percentage of total supply.
    initial_supply_percentage: Option<f64>,

    /// Annual percentage increase in supply, if supply is inflationary.
    inflation_rate: Option<f64>,

    /// Percentage of tokens burned during each transaction, if deflationary.
    burn_rate: Option<f64>,

    /// Initial price of the token in simulation
    initial_price: Option<f64>,

    /// Airdrop amount of the token, in percentage of total supply.
    airdrop_percentage: Option<f64>,

    /// Unlock schedule.
    unlock_schedule: Option<Vec<UnlockEvent>>,
}

impl TokenBuilder {
    /// Create a new token builder to configure the token.
    ///
    /// # Returns
    ///
    /// New token builder.
    pub fn new() -> Self {
        TokenBuilder::default()
    }

    /// Set the name of the token.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the token.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the symbol of the token.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Symbol of the token.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set the total supply of the token.
    ///
    /// # Arguments
    ///
    /// * `total_supply` - Total supply of the token.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn total_supply(mut self, total_supply: i64) -> Self {
        self.total_supply = Some(total_supply);
        self
    }

    /// Set the current supply of the token.
    ///
    /// # Arguments
    ///
    /// * `current_supply` - Current supply of the token.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn current_supply(mut self, current_supply: f64) -> Self {
        self.current_supply = Some(current_supply);
        self
    }

    /// Set the initial supply of the token, in percentage of total supply.
    ///
    /// # Arguments
    ///
    /// * `initial_supply_percentage` - Initial supply of the token.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn initial_supply_percentage(mut self, initial_supply_percentage: f64) -> Self {
        self.initial_supply_percentage = Some(initial_supply_percentage);
        self
    }

    /// Set the annual percentage increase in supply, if supply is inflationary.
    ///
    /// # Arguments
    ///
    /// * `inflation_rate` - Annual percentage increase in supply.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn inflation_rate(mut self, inflation_rate: f64) -> Self {
        self.inflation_rate = Some(inflation_rate);
        self
    }

    /// Set the percentage of tokens burned during each transaction, if deflationary.
    ///
    /// # Arguments
    ///
    /// * `burn_rate` - Percentage of tokens burned during each transaction.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn burn_rate(mut self, burn_rate: f64) -> Self {
        self.burn_rate = Some(burn_rate);
        self
    }

    /// Set the initial price of the token in simulation.
    ///
    ///
    /// # Arguments
    ///
    /// * `initial_price` - Initial price of the token.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn initial_price(mut self, initial_price: f64) -> Self {
        self.initial_price = Some(initial_price);
        self
    }

    /// Set the airdrop amount of the token, in percentage of total supply.
    ///
    /// # Arguments
    ///
    /// * `airdrop_percentage` - Airdrop amount of the token.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn airdrop_percentage(mut self, airdrop_percentage: f64) -> Self {
        self.airdrop_percentage = Some(airdrop_percentage);
        self
    }

    /// Set the unlock schedule.
    ///
    /// # Arguments
    ///
    /// * `unlock_schedule` - List of unlock events.
    ///
    /// # Returns
    ///
    /// The token builder.
    pub fn unlock_schedule(mut self, unlock_schedule: Vec<UnlockEvent>) -> Self {
        self.unlock_schedule = Some(unlock_schedule);
        self
    }

    /// Build the token.
    ///
    /// # Returns
    ///
    /// Token with the configured parameters.
    pub fn build(self) -> Result<Token, SimulationError> {
        Ok(Token {
            id: Uuid::new_v4(),
            name: self.name.ok_or(SimulationError::MissingName)?,
            symbol: self.symbol.unwrap_or_else(|| "TKN".to_string()),
            total_supply: match self.total_supply {
                Some(supply) => Decimal::from_i64(supply).ok_or(SimulationError::InvalidDecimal)?,
                None => Decimal::new(1_000_000, 0),
            },
            current_supply: match self.current_supply {
                Some(supply) => Decimal::from_f64(supply).ok_or(SimulationError::InvalidDecimal)?,
                None => Decimal::new(0, 0),
            },
            initial_supply_percentage: match self.initial_supply_percentage {
                Some(percentage) => {
                    Decimal::from_f64(percentage).ok_or(SimulationError::InvalidDecimal)?
                }
                None => Decimal::new(100, 0),
            },
            inflation_rate: match self.inflation_rate {
                Some(rate) => Some(Decimal::from_f64(rate).ok_or(SimulationError::InvalidDecimal)?),
                None => None,
            },
            burn_rate: match self.burn_rate {
                Some(rate) => Some(Decimal::from_f64(rate).ok_or(SimulationError::InvalidDecimal)?),
                None => None,
            },
            initial_price: match self.initial_price {
                Some(price) => Decimal::from_f64(price).ok_or(SimulationError::InvalidDecimal)?,
                None => Decimal::new(1, 0),
            },
            airdrop_percentage: match self.airdrop_percentage {
                Some(percentage) => {
                    Some(Decimal::from_f64(percentage).ok_or(SimulationError::InvalidDecimal)?)
                }
                None => None,
            },
            unlock_schedule: self.unlock_schedule,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_token_builder_with_defaults() {
        let token = TokenBuilder::new()
            .name("Test Token".to_string())
            .build()
            .unwrap();

        assert_eq!(token.name, "Test Token");
        assert_eq!(token.symbol, "TKN");
        assert_eq!(token.total_supply, Decimal::new(1_000_000, 0));
        assert_eq!(token.current_supply, Decimal::new(0, 0));
        assert_eq!(token.initial_supply_percentage, Decimal::new(100, 0));
        assert_eq!(token.inflation_rate, None);
        assert_eq!(token.burn_rate, None);
        assert_eq!(token.initial_price, Decimal::new(1, 0));
        assert_eq!(token.airdrop_percentage, None);
        assert!(token.unlock_schedule.is_none());
    }

    #[test]
    fn test_token_builder() {
        let unlock_event = UnlockEvent {
            date: Utc::now(),
            amount: Decimal::new(100_000, 0),
        };

        let token = TokenBuilder::new()
            .name("Test Token".to_string())
            .symbol("TT".to_string())
            .total_supply(1_000_000)
            .current_supply(100_000.0)
            .initial_supply_percentage(50.0)
            .inflation_rate(5.0)
            .burn_rate(1.0)
            .initial_price(2.0)
            .airdrop_percentage(10.0)
            .unlock_schedule(vec![unlock_event])
            .build()
            .unwrap();

        assert_eq!(token.name, "Test Token");
        assert_eq!(token.symbol, "TT");
        assert_eq!(token.total_supply, Decimal::new(1_000_000, 0));
        assert_eq!(token.current_supply, Decimal::new(100_000, 0));
        assert_eq!(token.initial_supply_percentage, Decimal::new(50, 0));
        assert_eq!(token.inflation_rate, Some(Decimal::new(5, 0)));
        assert_eq!(token.burn_rate, Some(Decimal::new(1, 0)));
        assert_eq!(token.initial_price, Decimal::new(2, 0));
        assert_eq!(token.airdrop_percentage, Some(Decimal::new(10, 0)));
        assert_eq!(token.unlock_schedule.unwrap().len(), 1);
    }

    #[test]
    fn test_token_builder_missing_name() {
        let token = TokenBuilder::new().build();

        assert_eq!(token, Err(SimulationError::MissingName));
    }
}
