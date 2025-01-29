//! # Report module
//!
//! This module contains the simulation report struct and its methods.
//! The simulation report contains the results of a simulation.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{User, DECIMAL_PRECISION};

/// Report containing the results of a simulation.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulationReport {
    /// Profit or loss for the interval.
    /// Positive value indicates profit, negative value indicates loss.
    pub profit_loss: Decimal,

    /// Number of trades made in the interval.
    /// This includes both successful and failed trades.
    pub trades: u64,

    /// Number of successful trades made in the interval.
    /// A trade is considered successful if the user has a positive balance.
    pub successful_trades: u64,

    /// Number of failed trades made in the interval.
    /// A trade is considered failed if the user has a zero balance.
    pub failed_trades: u64,

    /// Token distribution among participants.
    /// This is a list of token balances for each user.
    pub token_distribution: Vec<Decimal>,

    /// Market volatility during the simulation.
    /// This is the standard deviation of token prices.
    pub market_volatility: Decimal,

    /// Liquidity of the token during the simulation.
    /// Liquidity is the number of trades per second.
    pub liquidity: Decimal,

    /// Adoption rate of the token.
    /// Adoption rate is the percentage of users who have a positive balance.
    pub adoption_rate: Decimal,

    /// Burn rate of the token.
    /// Burn rate is the number of tokens burned per user.
    pub burn_rate: Decimal,

    /// Inflation rate of the token.
    /// Inflation rate is the number of new tokens created per user.
    pub inflation_rate: Decimal,

    /// User retention rate.
    /// User retention rate is the percentage of users who have a positive balance.
    pub user_retention: Decimal,

    /// Network activity (e.g., transactions per second).
    /// This is the number of transactions made in the interval.
    pub network_activity: u64,

    /// Actual token price during the simulation.
    /// This is the price of the token at the end of the simulation.
    pub token_price: Decimal,
}

impl Default for SimulationReport {
    /// Create a new simulation report with default values.
    ///
    /// # Returns
    ///
    /// A new simulation report with default values.
    fn default() -> Self {
        Self {
            profit_loss: Decimal::default(),
            trades: 0,
            successful_trades: 0,
            failed_trades: 0,
            token_distribution: vec![],
            market_volatility: Decimal::default(),
            liquidity: Decimal::default(),
            adoption_rate: Decimal::default(),
            burn_rate: Decimal::default(),
            inflation_rate: Decimal::default(),
            user_retention: Decimal::default(),
            token_price: Decimal::default(),
            network_activity: 0,
        }
    }
}

impl SimulationReport {
    /// Calculate the liquidity of the token.
    /// Liquidity is the number of trades per second.
    ///
    /// # Arguments
    ///
    /// * `trades` - Number of trades made in the interval.
    /// * `interval_duration` - Duration of the interval in seconds.
    ///
    /// # Returns
    ///
    /// The liquidity of the token as trades per second.
    pub fn calculate_liquidity(&self, trades: Decimal, interval_duration: Decimal) -> Decimal {
        (trades / interval_duration).round_dp(DECIMAL_PRECISION)
    }

    /// Calculate the adoption rate.
    /// Adoption rate is the percentage of users who have a positive balance.
    ///
    /// # Arguments
    ///
    /// * `users` - A list of users.
    ///
    /// # Returns
    ///
    /// The adoption rate as a percentage.
    pub fn calculate_adoption_rate(&self, users: &[User]) -> Decimal {
        let total_users = Decimal::new(users.len() as i64, 0);
        let new_users = Decimal::new(
            users
                .iter()
                .filter(|u| u.balance > Decimal::default())
                .count() as i64,
            0,
        );

        (new_users / total_users).round_dp(DECIMAL_PRECISION)
    }

    /// Calculate the burn rate.
    /// Burn rate is the number of tokens burned per user.
    ///
    /// # Arguments
    ///
    /// * `total_burned` - Total number of tokens burned.
    /// * `total_users` - Total number of users.
    ///
    /// # Returns
    ///
    /// The burn rate as a percentage.
    pub fn calculate_burn_rate(&self, total_burned: Decimal, total_users: Decimal) -> Decimal {
        (total_burned / total_users).round_dp(DECIMAL_PRECISION)
    }

    /// Calculate the inflation rate.
    /// Inflation rate is the number of new tokens created per user.
    ///
    /// # Arguments
    ///
    /// * `total_new_tokens` - Total number of new tokens created.
    /// * `total_users` - Total number of users.
    ///
    /// # Returns
    ///
    /// The inflation rate as a percentage.
    pub fn calculate_inflation_rate(
        &self,
        total_new_tokens: Decimal,
        total_users: Decimal,
    ) -> Decimal {
        (total_new_tokens / total_users).round_dp(DECIMAL_PRECISION)
    }

    /// Calculate the user retention rate.
    /// User retention rate is the percentage of users who have a positive balance.
    ///
    /// # Arguments
    ///
    /// * `users` - A list of users.
    ///
    /// # Returns
    ///
    /// The user retention rate as a percentage.
    pub fn calculate_user_retention(&self, users: &[User]) -> Decimal {
        let total_users = Decimal::new(users.len() as i64, 0);
        let retained_users = Decimal::new(
            users
                .iter()
                .filter(|u| u.balance > Decimal::default())
                .count() as i64,
            0,
        );

        (retained_users / total_users).round_dp(DECIMAL_PRECISION)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_default() {
        let report = SimulationReport::default();

        assert_eq!(report.profit_loss, Decimal::default());
        assert_eq!(report.trades, 0);
        assert_eq!(report.successful_trades, 0);
        assert_eq!(report.failed_trades, 0);
        assert!(report.token_distribution.is_empty());
        assert_eq!(report.market_volatility, Decimal::default());
        assert_eq!(report.liquidity, Decimal::default());
        assert_eq!(report.adoption_rate, Decimal::default());
        assert_eq!(report.burn_rate, Decimal::default());
        assert_eq!(report.inflation_rate, Decimal::default());
        assert_eq!(report.user_retention, Decimal::default());
        assert_eq!(report.network_activity, 0);
    }

    #[test]
    fn test_calculate_liquidity() {
        let report = SimulationReport::default();
        let trades = Decimal::new(100, 0);
        let interval_duration = Decimal::new(10, 0);

        assert_eq!(
            report.calculate_liquidity(trades, interval_duration),
            Decimal::new(10, 0)
        );
    }

    #[test]
    fn test_calculate_adoption_rate() {
        let report = SimulationReport::default();
        let users = vec![
            User::new(Uuid::new_v4(), Decimal::default()),
            User::new(Uuid::new_v4(), Decimal::new(10, 0)),
            User::new(Uuid::new_v4(), Decimal::default()),
            User::new(Uuid::new_v4(), Decimal::new(5, 0)),
        ];

        assert_eq!(report.calculate_adoption_rate(&users), Decimal::new(5, 1));
    }

    #[test]
    fn test_calculate_burn_rate() {
        let report = SimulationReport::default();
        let total_burned = Decimal::new(100, 0);
        let total_users = Decimal::new(10, 0);

        assert_eq!(
            report.calculate_burn_rate(total_burned, total_users),
            Decimal::new(10, 0)
        );
    }

    #[test]
    fn test_calculate_inflation_rate() {
        let report = SimulationReport::default();
        let total_new_tokens = Decimal::new(100, 0);
        let total_users = Decimal::new(10, 0);

        assert_eq!(
            report.calculate_inflation_rate(total_new_tokens, total_users),
            Decimal::new(10, 0)
        );
    }

    #[test]
    fn test_calculate_user_retention() {
        let report = SimulationReport::default();
        let users = vec![
            User::new(Uuid::new_v4(), Decimal::default()),
            User::new(Uuid::new_v4(), Decimal::new(10, 0)),
            User::new(Uuid::new_v4(), Decimal::default()),
            User::new(Uuid::new_v4(), Decimal::new(5, 0)),
        ];

        assert_eq!(report.calculate_user_retention(&users), Decimal::new(5, 1));
    }
}
