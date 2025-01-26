use chrono::{DateTime, Utc};
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

    /// Maximum supply of the token.
    pub maximum_supply: u64,

    /// Current supply of the token.
    pub current_supply: u64,

    /// Initial supply of the token, in percentage of maximum supply.
    pub initial_supply_percentage: f64,

    /// Annual percentage increase in supply, if supply is inflationary.
    pub inflation_rate: Option<f64>,

    /// Percentage of tokens burned during each transaction, if deflationary.
    pub burn_rate: Option<f64>,

    /// Initial price of the token in simulation
    pub initial_price: Option<f64>,

    /// Airdrop amount of the token, in percentage of maximum supply.
    pub airdrop_percentage: Option<f64>,

    /// Unlock schedule.
    pub unlock_schedule: Option<Vec<UnlockEvent>>,
}

/// Unlock event.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct UnlockEvent {
    /// Date and time of the unlock event.
    pub date: DateTime<Utc>,

    /// Amount of tokens to unlock.
    pub amount: u64,
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
            maximum_supply: 1_000_000,
            current_supply: 0,
            initial_supply_percentage: 100.0,
            inflation_rate: None,
            burn_rate: None,
            initial_price: Some(1.0),
            airdrop_percentage: None,
            unlock_schedule: None,
        }
    }
}

impl Token {
    /// Perform an airdrop.
    ///
    /// # Arguments
    ///
    /// * `percentage` - The percentage of the maximum supply to airdrop.
    ///
    /// # Returns
    ///
    /// The amount of tokens airdropped.
    pub fn airdrop(&mut self, percentage: f64) -> u64 {
        let airdrop_amount = (self.maximum_supply as f64 * percentage / 100.0).round() as u64;
        let remaining_supply = self.maximum_supply - self.current_supply;
        let final_airdrop_amount = if airdrop_amount > remaining_supply {
            remaining_supply
        } else {
            airdrop_amount
        };

        self.current_supply += final_airdrop_amount;

        final_airdrop_amount
    }

    /// Add an unlock event to the schedule.
    ///
    /// # Arguments
    ///
    /// * `date` - The date and time of the unlock event.
    /// * `amount` - The amount of tokens to unlock.
    pub fn add_unlock_event(&mut self, date: DateTime<Utc>, amount: u64) {
        let event = UnlockEvent { date, amount };

        if let Some(schedule) = &mut self.unlock_schedule {
            schedule.push(event);
        } else {
            self.unlock_schedule = Some(vec![event]);
        }
    }

    /// Process unlock events up to the current date.
    ///
    /// # Arguments
    ///
    /// * `current_date` - The current date and time.
    pub fn process_unlocks(&mut self, current_date: DateTime<Utc>) {
        if let Some(schedule) = &mut self.unlock_schedule {
            schedule.retain(|event| {
                if event.date <= current_date {
                    self.current_supply += event.amount;
                    false
                } else {
                    true
                }
            });
        }
    }

    /// Calculate the initial supply based on the initial supply percentage.
    ///
    /// # Returns
    ///
    /// Initial supply of the token.
    pub fn initial_supply(&self) -> u64 {
        (self.maximum_supply as f64 * self.initial_supply_percentage / 100.0).round() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_default() {
        let token = Token::default();

        assert_eq!(token.name, "Token");
        assert_eq!(token.symbol, Some("TKN".to_string()));
        assert_eq!(token.maximum_supply, 1_000_000);
        assert_eq!(token.current_supply, 0);
        assert_eq!(token.initial_supply_percentage, 100.0);
        assert_eq!(token.inflation_rate, None);
        assert_eq!(token.burn_rate, None);
        assert_eq!(token.initial_price, Some(1.0));
        assert_eq!(token.airdrop_percentage, None);
        assert_eq!(token.unlock_schedule, None);
    }

    #[test]
    fn test_token_airdrop() {
        let mut token = Token::default();

        let airdrop_amount = token.airdrop(10.0);

        assert_eq!(airdrop_amount, 100_000);
        assert_eq!(token.current_supply, 100_000);

        let airdrop_amount = token.airdrop(100.0);

        assert_eq!(airdrop_amount, 900_000);
        assert_eq!(token.current_supply, 1_000_000);
    }
}
