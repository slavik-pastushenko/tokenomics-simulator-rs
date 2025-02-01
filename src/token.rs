//! # Token module
//!
//! This module applies airdrops and unlock events to the token.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Token.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Token {
    /// ID for the token.
    pub id: Uuid,

    /// Name of the token.
    /// The name is a human-readable identifier for the token.
    pub name: String,

    /// Symbol of the token.
    /// The symbol is a short identifier for the token, usually 3-4 characters long.
    pub symbol: String,

    /// Total supply of the token.
    /// The total supply is the maximum number of tokens that can ever exist.
    pub total_supply: Decimal,

    /// Current supply of the token.
    /// The current supply is the number of tokens that have been minted or airdropped.
    pub current_supply: Decimal,

    /// Initial supply of the token, in percentage of total supply.
    /// The initial supply is the number of tokens that are minted at the start of the simulation.
    pub initial_supply_percentage: Decimal,

    /// Annual percentage increase in supply, if supply is inflationary.
    /// The inflation rate is the percentage by which the total supply increases each year.
    pub inflation_rate: Option<Decimal>,

    /// Percentage of tokens burned during each transaction, if deflationary.
    /// The burn rate is the percentage of tokens that are destroyed during each transaction.
    pub burn_rate: Option<Decimal>,

    /// Initial price of the token in simulation.
    /// The initial price is the price of the token at the start of the simulation.
    pub initial_price: Decimal,

    /// Airdrop amount of the token, in percentage of total supply.
    /// The airdrop percentage is the percentage of the total supply that is airdropped at the start of the simulation.
    pub airdrop_percentage: Option<Decimal>,

    /// Unlock schedule.
    /// The unlock schedule is a list of unlock events, each with a date and amount of tokens to unlock.
    pub unlock_schedule: Option<Vec<UnlockEvent>>,
}

/// Unlock event.
/// An unlock event is a scheduled event that unlocks a certain amount of tokens at a certain date.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct UnlockEvent {
    /// Date and time of the unlock event.
    pub date: DateTime<Utc>,

    /// Amount of tokens to unlock.
    pub amount: Decimal,
}

impl Token {
    /// Perform an airdrop.
    ///
    /// # Arguments
    ///
    /// * `percentage` - The percentage of the total supply to airdrop.
    ///
    /// # Returns
    ///
    /// The amount of tokens airdropped.
    pub fn airdrop(&mut self, percentage: Decimal) -> Decimal {
        let airdrop_amount = (self.total_supply * percentage / Decimal::new(100, 0)).round();
        let remaining_supply = self.total_supply - self.current_supply;
        let final_airdrop_amount = if airdrop_amount > remaining_supply {
            remaining_supply
        } else {
            airdrop_amount
        };

        self.current_supply += final_airdrop_amount;

        final_airdrop_amount
    }

    /// Add an unlock event to the schedule.
    /// The unlock event will unlock a certain amount of tokens at a certain date.
    ///
    /// # Arguments
    ///
    /// * `date` - The date and time of the unlock event.
    /// * `amount` - The amount of tokens to unlock.
    pub fn add_unlock_event(&mut self, date: DateTime<Utc>, amount: Decimal) {
        let event = UnlockEvent { date, amount };

        if let Some(schedule) = &mut self.unlock_schedule {
            schedule.push(event);
        } else {
            self.unlock_schedule = Some(vec![event]);
        }
    }

    /// Process unlock events up to the current date.
    /// Unlocks tokens and removes events that have already occurred.
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
    /// The initial supply is the number of tokens that are minted at the start of the simulation.
    ///
    /// # Returns
    ///
    /// Initial supply of the token.
    pub fn initial_supply(&self) -> Decimal {
        (self.total_supply * self.initial_supply_percentage / Decimal::new(100, 0)).round()
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenBuilder;

    use super::*;

    #[test]
    fn test_token_airdrop() {
        let mut token = TokenBuilder::new()
            .name("Test Token".to_string())
            .total_supply(1_000_000)
            .build()
            .unwrap();
        let final_amount = Decimal::new(100000, 0);

        let airdrop_amount = token.airdrop(Decimal::new(10, 0));

        assert_eq!(airdrop_amount, final_amount);
        assert_eq!(token.current_supply, final_amount);

        let airdrop_amount = token.airdrop(Decimal::new(100, 0));

        assert_eq!(airdrop_amount, Decimal::new(900000, 0));
        assert_eq!(token.current_supply, Decimal::new(1_000_000, 0));
    }

    #[test]
    fn test_add_unlock_event() {
        let mut token = TokenBuilder::new()
            .name("Test Token".to_string())
            .total_supply(1_000_000)
            .build()
            .unwrap();
        let date = Utc::now();
        let amount = Decimal::new(100000, 0);

        token.add_unlock_event(date, amount);
        token.add_unlock_event(date, amount);

        assert_eq!(token.unlock_schedule.unwrap().len(), 2);
    }

    #[test]
    fn test_process_unlock() {
        let mut token = TokenBuilder::new()
            .name("Test Token".to_string())
            .total_supply(1_000_000)
            .build()
            .unwrap();
        let date = Utc::now();
        let amount = Decimal::new(100000, 0);
        token.add_unlock_event(date, amount);

        let current_date = date + chrono::Duration::days(1);
        token.process_unlocks(current_date);

        assert_eq!(token.current_supply, amount);
        assert!(token.unlock_schedule.unwrap().is_empty());
    }
}
