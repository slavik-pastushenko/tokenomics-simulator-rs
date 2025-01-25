use serde::{Deserialize, Serialize};

use crate::User;

/// Report containing the results of a simulation.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulationReport {
    /// Profit or loss for the interval.
    pub profit_loss: f64,

    /// Number of trades made in the interval.
    pub trades: u64,

    /// Number of successful trades made in the interval.
    pub successful_trades: u64,

    /// Number of failed trades made in the interval.
    pub failed_trades: u64,

    /// Token distribution among participants.
    pub token_distribution: Vec<f64>,

    /// Market volatility during the simulation.
    pub market_volatility: f64,

    /// Liquidity of the token during the simulation.
    pub liquidity: f64,

    /// Adoption rate of the token.
    pub adoption_rate: f64,

    /// Burn rate of the token.
    pub burn_rate: f64,

    /// Inflation rate of the token.
    pub inflation_rate: f64,

    /// User retention rate.
    pub user_retention: f64,

    /// Network activity (e.g., transactions per second).
    pub network_activity: f64,
}

impl Default for SimulationReport {
    /// Create a new simulation report with default values.
    ///
    /// # Returns
    ///
    /// A new simulation report with default values.
    fn default() -> Self {
        Self {
            profit_loss: 0.0,
            trades: 0,
            successful_trades: 0,
            failed_trades: 0,
            token_distribution: vec![],
            market_volatility: 0.0,
            liquidity: 0.0,
            adoption_rate: 0.0,
            burn_rate: 0.0,
            inflation_rate: 0.0,
            user_retention: 0.0,
            network_activity: 0.0,
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
    pub fn calculate_liquidity(&self, trades: u64, interval_duration: u64) -> f64 {
        trades as f64 / interval_duration as f64
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
    pub fn calculate_adoption_rate(&self, users: &[User]) -> f64 {
        let total_users = users.len();
        let new_users = users.iter().filter(|u| u.balance > 0.0).count();

        new_users as f64 / total_users as f64
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
    pub fn calculate_burn_rate(&self, total_burned: f64, total_users: u64) -> f64 {
        total_burned / total_users as f64
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
    pub fn calculate_inflation_rate(&self, total_new_tokens: f64, total_users: u64) -> f64 {
        total_new_tokens / total_users as f64
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
    pub fn calculate_user_retention(&self, users: &[User]) -> f64 {
        let total_users = users.len();
        let retained_users = users.iter().filter(|u| u.balance > 0.0).count();

        retained_users as f64 / total_users as f64
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_default() {
        let report = SimulationReport::default();

        assert_eq!(report.profit_loss, 0.0);
        assert_eq!(report.trades, 0);
        assert_eq!(report.successful_trades, 0);
        assert_eq!(report.failed_trades, 0);
        assert!(report.token_distribution.is_empty());
        assert_eq!(report.market_volatility, 0.0);
        assert_eq!(report.liquidity, 0.0);
        assert_eq!(report.adoption_rate, 0.0);
        assert_eq!(report.burn_rate, 0.0);
        assert_eq!(report.inflation_rate, 0.0);
        assert_eq!(report.user_retention, 0.0);
        assert_eq!(report.network_activity, 0.0);
    }

    #[test]
    fn test_calculate_liquidity() {
        let report = SimulationReport::default();
        let trades = 100;
        let interval_duration = 10;

        assert_eq!(report.calculate_liquidity(trades, interval_duration), 10.0);
    }

    #[test]
    fn test_calculate_adoption_rate() {
        let report = SimulationReport::default();
        let users = vec![
            User::new(Uuid::new_v4(), 0.0),
            User::new(Uuid::new_v4(), 10.0),
            User::new(Uuid::new_v4(), 0.0),
            User::new(Uuid::new_v4(), 5.0),
        ];

        assert_eq!(report.calculate_adoption_rate(&users), 0.5);
    }

    #[test]
    fn test_calculate_burn_rate() {
        let report = SimulationReport::default();
        let total_burned = 100.0;
        let total_users = 10;

        assert_eq!(report.calculate_burn_rate(total_burned, total_users), 10.0);
    }

    #[test]
    fn test_calculate_inflation_rate() {
        let report = SimulationReport::default();
        let total_new_tokens = 100.0;
        let total_users = 10;

        assert_eq!(
            report.calculate_inflation_rate(total_new_tokens, total_users),
            10.0
        );
    }

    #[test]
    fn test_calculate_user_retention() {
        let report = SimulationReport::default();
        let users = vec![
            User::new(Uuid::new_v4(), 0.0),
            User::new(Uuid::new_v4(), 10.0),
            User::new(Uuid::new_v4(), 0.0),
            User::new(Uuid::new_v4(), 5.0),
        ];

        assert_eq!(report.calculate_user_retention(&users), 0.5);
    }
}
