//! # Engine module
//!
//! The engine module contains the core logic for the tokenomics simulation.
//!
//! This module provides the simulation struct and related types to simulate the tokenomics of a token.
//! The simulation contains the input parameters, token, and reports for the simulation.

use chrono::{DateTime, Utc};
use rand::Rng;
use rust_decimal::{prelude::*, Decimal, MathematicalOps};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    SimulationBuilder, SimulationError, SimulationOptions, SimulationOptionsBuilder,
    SimulationReport, Token, TokenBuilder, User, ValuationModel,
};

/// Simulation.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Simulation {
    /// ID of the simulation.
    pub id: Uuid,

    /// Name of the simulation.
    /// This is used to identify the simulation.
    pub name: String,

    /// Token used in the simulation.
    /// This token is used to simulate the tokenomics.
    pub token: Token,

    /// Description of the simulation.
    /// This is used to provide additional information about the simulation.
    pub description: Option<String>,

    /// Status of the simulation.
    /// Default is `SimulationStatus::Pending`.
    pub status: SimulationStatus,

    /// Input parameters for the simulation.
    /// These parameters are used to configure the simulation.
    pub options: SimulationOptions,

    /// Report of the results for each interval of the simulation.
    /// This is used to track the progress of the simulation.
    pub interval_reports: Vec<SimulationReport>,

    /// Report of the total results of the simulation.
    /// This is used to provide a summary of the simulation.
    pub report: SimulationReport,

    /// Date and time the simulation was created.
    pub created_at: DateTime<Utc>,

    /// Date and time the simulation was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Status of a simulation.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SimulationStatus {
    /// Simulation has not started.
    Pending,

    /// Simulation is currently running.
    Running,

    /// Simulation has completed.
    Completed,
}

/// Interval type for the simulation.
/// This is used to determine the duration of each interval in the simulation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SimulationInterval {
    /// Hourly interval.
    Hourly,

    /// Daily interval.
    Daily,

    /// Weekly interval.
    Weekly,

    /// Monthly interval.
    Monthly,
}

impl Simulation {
    /// Create a new simulation with the given token and options.
    ///
    /// # Returns
    ///
    /// New simulation builder.
    pub fn builder() -> SimulationBuilder {
        SimulationBuilder::new()
    }

    /// Create a new simulation options builder to configure the simulation.
    ///
    /// # Returns
    ///
    /// New simulation options builder.
    pub fn options_builder() -> SimulationOptionsBuilder {
        SimulationOptionsBuilder::new()
    }

    /// Create a new token builder to configure the token used in the simulation.
    ///
    /// # Returns
    ///
    /// New token builder.
    pub fn token_builder() -> TokenBuilder {
        TokenBuilder::new()
    }

    /// Update the status of the simulation.   
    ///
    /// # Arguments
    ///
    /// * `status` - The new status of the simulation.
    pub fn update_status(&mut self, status: SimulationStatus) {
        #[cfg(feature = "log")]
        log::debug!("Updating simulation status: {:?}", status);

        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Simulate user adoption based on the current number of users and the adoption rate.
    /// If the adoption rate is not set, the current number of users is returned.
    ///
    /// # Arguments
    ///
    /// * `current_users` - The current number of users.
    ///
    /// # Returns
    ///
    /// The new number of users after adoption.
    pub fn simulate_adoption(&self, current_users: u64) -> Result<u64, SimulationError> {
        match self.options.adoption_rate {
            Some(rate) => {
                #[cfg(feature = "log")]
                log::debug!("Simulating user adoption for simulation: {}", self.name);

                let new_users = (current_users as f64
                    * rate.to_f64().ok_or(SimulationError::InvalidDecimal)?)
                .round() as u64;

                let total = current_users + new_users;

                #[cfg(feature = "log")]
                log::debug!("User adoption simulated: {}", total);

                Ok(total)
            }
            None => Ok(current_users),
        }
    }

    /// Calculate the valuation of the token based on the number of users and the initial price.
    /// The valuation model is used to determine how the valuation is calculated.
    /// If the valuation model is not set, the default valuation is returned.
    ///
    /// # Arguments
    ///
    /// * `token` - The token used in the simulation.
    /// * `users` - The current number of users.
    ///
    /// # Returns
    ///
    /// The calculated token valuation.
    pub fn calculate_valuation(&self, token: &Token, users: u64) -> Decimal {
        match self.options.valuation_model {
            Some(ValuationModel::Linear) => {
                #[cfg(feature = "log")]
                log::debug!("Calculating linear valuation for simulation: {}", self.name);

                let valuation = Decimal::from(users) * token.initial_price;

                #[cfg(feature = "log")]
                log::debug!("Linear valuation calculated: {}", valuation);

                valuation
            }
            Some(ValuationModel::Exponential(factor)) => {
                #[cfg(feature = "log")]
                log::debug!("Calculating exponential valuation with factor: {}", factor);

                let exponent = match Decimal::from_f64(factor) {
                    Some(factor) => Decimal::from(users) / factor,
                    None => Decimal::from(users),
                };

                let valuation = match exponent.checked_exp() {
                    Some(exp) => token.initial_price * exp,
                    None => token.initial_price,
                };

                #[cfg(feature = "log")]
                log::debug!("Exponential valuation calculated: {}", valuation);

                valuation
            }
            _ => Decimal::default(),
        }
    }

    /// Run the simulation.
    /// This will simulate the tokenomics based on the input parameters.
    /// The simulation will run for the specified duration and generate reports for each interval.
    /// The final report will be generated at the end of the simulation.
    ///
    /// # Returns
    ///
    /// Result of the simulation.
    pub fn run(&mut self) -> Result<(), SimulationError> {
        #[cfg(feature = "log")]
        log::debug!("Running simulation: {}", self.name);

        self.update_status(SimulationStatus::Running);

        let decimal_precision = self.options.decimal_precision;

        #[cfg(feature = "log")]
        log::debug!(
            "Generating initial user distribution for simulation: {}",
            self.name
        );

        let airdrop_amount = match self.token.airdrop_percentage {
            Some(percentage) => self.token.airdrop(percentage),
            None => Decimal::default(),
        };

        let mut users = User::generate(
            self.options.total_users,
            self.token.initial_supply(),
            self.token.initial_price,
            decimal_precision,
        );

        #[cfg(feature = "log")]
        log::debug!("Initial user distribution generated");

        // Distribute airdrop amount among users, if available
        if !airdrop_amount.is_zero() {
            #[cfg(feature = "log")]
            log::debug!("Distributing airdrop amount: {}", airdrop_amount);

            let airdrop_per_user = airdrop_amount / Decimal::new(users.len() as i64, 0);

            #[cfg(feature = "log")]
            log::debug!("Airdrop amount per user: {}", airdrop_per_user);

            for user in &mut users {
                user.balance += airdrop_per_user.round_dp(decimal_precision);
            }

            #[cfg(feature = "log")]
            log::debug!("Airdrop amount distributed");
        }

        self.interval_reports = vec![];

        let interval = self.get_interval();

        #[cfg(feature = "log")]
        log::debug!("Simulation interval: {}", interval);

        for time in (0..self.options.duration * interval).step_by(interval as usize) {
            #[cfg(feature = "log")]
            log::debug!("Processing interval: {}", time);

            // Process unlock events up to the current time
            let current_date = Utc::now() + chrono::Duration::hours(time as i64);
            self.token.process_unlocks(current_date);

            // Simulate user adoption
            let current_users = self.simulate_adoption(users.len() as u64)?;
            users = User::generate(
                current_users,
                self.token.initial_supply(),
                self.token.initial_price,
                decimal_precision,
            );

            let valuation = self.calculate_valuation(&self.token, current_users);
            let mut report = self.process_interval(&mut users, interval)?;
            report.token_price = valuation;
            report.interval = current_date.timestamp_millis();

            self.interval_reports.push(report);

            #[cfg(feature = "log")]
            log::debug!("Interval processed: {}", time);
        }

        self.generate_final_report(users);
        self.update_status(SimulationStatus::Completed);

        #[cfg(feature = "log")]
        log::debug!("Simulation completed: {}", self.name);

        Ok(())
    }

    /// Simulate trades for a given interval.
    /// This will simulate trades for each user in the list and generate a report for the interval.
    ///
    /// # Arguments
    ///
    /// * `users` - A list of users.
    /// * `interval` - Duration of the interval.
    ///
    /// # Returns
    ///
    /// A report of the simulation results for the interval.
    pub fn process_interval(
        &self,
        users: &mut [User],
        interval: u64,
    ) -> Result<SimulationReport, SimulationError> {
        let mut rng = rand::rng();

        let decimal_precision = self.options.decimal_precision;
        let mut total_burned = Decimal::default();
        let mut total_new_tokens = Decimal::default();
        let mut report = SimulationReport::default();

        for _ in 0..interval {
            for user in users.iter_mut() {
                // Skip users with zero balance
                if user.balance.is_zero() {
                    continue;
                }

                if rng.random_bool(0.5) {
                    // Simulate a successful trade and randomize the fraction between 1% and 10% of the user's balance
                    let trade_fraction = rng.random_range(0.01..0.1);
                    let max_trade_amount = user
                        .balance
                        .to_f64()
                        .ok_or(SimulationError::InvalidDecimal)?
                        * trade_fraction;

                    // Ensure the range is valid
                    if max_trade_amount > 0.0 {
                        let trade_amount =
                            Decimal::from_f64(rng.random_range(0.0..max_trade_amount))
                                .ok_or(SimulationError::InvalidDecimal)?
                                .round_dp(decimal_precision);

                        user.balance -= trade_amount;
                        report.profit_loss += trade_amount;
                        report.successful_trades += 1;

                        if let Some(burn_rate) = self.token.burn_rate {
                            let burned = trade_amount * burn_rate;
                            user.balance -= burned;
                            total_burned += burned;
                        }

                        if let Some(inflation_rate) = self.token.inflation_rate {
                            let new_tokens = trade_amount * inflation_rate;
                            user.balance += new_tokens;
                            total_new_tokens += new_tokens;
                        }

                        if let Some(fee) = self.options.transaction_fee_percentage {
                            let fee = trade_amount * (fee / Decimal::new(100, 0));
                            user.balance -= fee.round_dp(decimal_precision);
                        }
                    } else {
                        report.failed_trades += 1;
                    }
                } else {
                    report.failed_trades += 1;
                }
            }
        }

        self.generate_interval_report(users, &mut report, interval);

        Ok(report)
    }

    /// Generate the interval report for the simulation.
    ///
    /// # Arguments
    ///
    /// * `users` - A list of users.
    /// * `report` - The simulation report for the interval.
    /// * `interval` - Duration of the interval.
    pub fn generate_interval_report(
        &self,
        users: &[User],
        report: &mut SimulationReport,
        interval: u64,
    ) {
        #[cfg(feature = "log")]
        log::debug!("Generating interval report for simulation: {}", self.name);

        let decimal_precision = self.options.decimal_precision;

        report.trades = report.successful_trades + report.failed_trades;
        report.liquidity = report.calculate_liquidity(
            Decimal::new(report.trades as i64, 0),
            Decimal::new(interval as i64, 0),
            decimal_precision,
        );
        report.adoption_rate = report.calculate_adoption_rate(users, decimal_precision);
        report.burn_rate = report.calculate_burn_rate(
            report.total_burned,
            Decimal::new(users.len() as i64, 0),
            decimal_precision,
        );
        report.user_retention = report.calculate_user_retention(users, decimal_precision);
        report.market_volatility = self.options.market_volatility;
        report.network_activity = report.trades / interval;
        report.inflation_rate = report.calculate_inflation_rate(
            report.total_new_tokens,
            Decimal::new(users.len() as i64, 0),
            decimal_precision,
        );

        #[cfg(feature = "log")]
        log::debug!("Interval report generated for simulation: {}", self.name);
    }

    /// Calculate the final report for the simulation.
    /// This will generate a summary of the simulation results based on the interval reports.
    ///
    /// # Arguments
    ///
    /// * `users` - A list of users.
    pub fn generate_final_report(&mut self, users: Vec<User>) {
        #[cfg(feature = "log")]
        log::debug!("Generating final report for simulation: {}", self.name);

        let mut report = SimulationReport {
            market_volatility: self.options.market_volatility,
            ..Default::default()
        };

        let mut total_burned = Decimal::default();
        let mut total_new_tokens = Decimal::default();
        let mut total_token_price = Decimal::default();
        let decimal_precision = self.options.decimal_precision;
        let total_users = Decimal::new(self.options.total_users as i64, 0);

        #[cfg(feature = "log")]
        log::debug!("Total interval reports: {}", self.interval_reports.len());

        for result in self.interval_reports.iter() {
            report.profit_loss += result.profit_loss;
            report.trades += result.trades;
            report.successful_trades += result.successful_trades;
            report.failed_trades += result.failed_trades;

            total_burned += result.burn_rate * total_users;
            total_new_tokens += result.inflation_rate * total_users;
            report.liquidity += result.liquidity;
            report.adoption_rate += result.adoption_rate;
            report.user_retention += result.user_retention;
            total_token_price += result.token_price;
        }

        let total_trades = Decimal::new(report.trades as i64, 0);
        let total_intervals = Decimal::new(self.interval_reports.len() as i64, 0);

        report.liquidity = (report.liquidity / total_intervals).round_dp(decimal_precision);
        report.adoption_rate = (report.adoption_rate / total_intervals).round_dp(decimal_precision);
        report.user_retention =
            (report.user_retention / total_intervals).round_dp(decimal_precision);
        report.burn_rate =
            report.calculate_burn_rate(total_burned, total_trades, self.options.decimal_precision);
        report.inflation_rate = (total_new_tokens / total_trades).round_dp(decimal_precision);
        report.network_activity = report.trades / self.options.duration;
        report.token_price = (total_token_price / total_intervals).round_dp(decimal_precision);
        report.users = Some(users);

        self.report = report;

        #[cfg(feature = "log")]
        log::debug!("Final report generated for simulation: {}", self.name);
    }

    /// Get the interval for the simulation.
    ///
    /// # Returns
    ///
    /// The duration of the simulation interval.
    pub fn get_interval(&self) -> u64 {
        match self.options.interval_type {
            SimulationInterval::Hourly => 1,
            SimulationInterval::Daily => 24,
            SimulationInterval::Weekly => 24 * 7,
            SimulationInterval::Monthly => 24 * 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Simulation {
        let token = Simulation::token_builder()
            .name("Test Token".to_string())
            .total_supply(1_000_000)
            .build()
            .unwrap();

        Simulation {
            id: Uuid::new_v4(),
            name: "Test Simulation".to_string(),
            token,
            description: None,
            status: SimulationStatus::Running,
            options: SimulationOptions {
                duration: 30,
                total_users: 100,
                decimal_precision: 4,
                market_volatility: Decimal::new(5, 1),
                transaction_fee_percentage: None,
                interval_type: SimulationInterval::Daily,
                adoption_rate: None,
                valuation_model: Some(ValuationModel::Exponential(0.1)),
            },
            interval_reports: vec![],
            report: SimulationReport::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_builder() {
        let builder = Simulation::builder();
        assert_eq!(builder, SimulationBuilder::new());
    }

    #[test]
    fn test_options_builder() {
        let builder = Simulation::options_builder();
        assert_eq!(builder, SimulationOptionsBuilder::new());
    }

    #[test]
    fn test_token_builder() {
        let builder = Simulation::token_builder();
        assert_eq!(builder, TokenBuilder::new());
    }

    #[test]
    fn test_get_interval() {
        let daily_simulation = setup();
        assert_eq!(daily_simulation.get_interval(), 24);

        let mut hourly_simulation = setup();
        hourly_simulation.options.interval_type = SimulationInterval::Hourly;
        assert_eq!(hourly_simulation.get_interval(), 1);

        let mut weekly_simulation = setup();
        weekly_simulation.options.interval_type = SimulationInterval::Weekly;
        assert_eq!(weekly_simulation.get_interval(), 24 * 7);

        let mut monthly_simulation = setup();
        monthly_simulation.options.interval_type = SimulationInterval::Monthly;
        assert_eq!(monthly_simulation.get_interval(), 24 * 30);
    }

    #[test]
    fn test_update_status() {
        let mut simulation = setup();
        simulation.update_status(SimulationStatus::Completed);

        assert_eq!(simulation.status, SimulationStatus::Completed);
    }

    #[test]
    fn test_run_with_valid_exponential_factor() {
        let mut simulation = setup();
        simulation.options.valuation_model = Some(ValuationModel::Exponential(1.0));

        simulation.run().unwrap();

        assert_eq!(simulation.status, SimulationStatus::Completed);
        assert_eq!(simulation.interval_reports.len(), 30);
    }

    #[test]
    fn test_run_with_airdrop() {
        let mut simulation = setup();
        simulation.token.airdrop_percentage = Some(Decimal::new(10, 0));

        simulation.run().unwrap();

        assert_eq!(simulation.status, SimulationStatus::Completed);
        assert_eq!(simulation.interval_reports.len(), 30);
        assert_eq!(simulation.report.users.unwrap().len(), 100);
    }

    #[test]
    fn test_calculate_valuation_linear() {
        let mut simulation = setup();
        simulation.token.initial_price = Decimal::new(1, 2);
        simulation.options.valuation_model = Some(ValuationModel::Linear);

        let token = &simulation.token;
        let users = 99;
        let valuation = simulation.calculate_valuation(token, users);

        assert_eq!(valuation, Decimal::new(99, 2));
    }

    #[test]
    fn test_calculate_valuation_exponential() {
        let mut simulation = setup();
        simulation.options.valuation_model = Some(ValuationModel::Exponential(2.0));

        let token = &simulation.token;
        let users = 100;
        let valuation = simulation.calculate_valuation(token, users);

        assert_eq!(valuation, Decimal::new(1, 0));
    }

    #[test]
    fn test_calculate_valuation_exponential_overflow() {
        let mut simulation = setup();
        simulation.options.valuation_model = Some(ValuationModel::Exponential(0.1));

        let token = &simulation.token;
        let users = 1_000_000;
        let valuation = simulation.calculate_valuation(token, users);

        assert_eq!(valuation, Decimal::new(1, 0));
    }

    #[test]
    fn test_calculate_valuation_default() {
        let mut simulation = setup();
        simulation.options.valuation_model = None;

        let token = &simulation.token;
        let users = 1_000_000;
        let valuation = simulation.calculate_valuation(token, users);

        assert_eq!(valuation, Decimal::default());
    }

    #[test]
    fn test_simulate_adoption_with_rate() {
        let simulation = setup();
        let current_users = 100;

        let new_users = simulation.simulate_adoption(current_users).unwrap();
        assert_eq!(new_users, 100);

        let simulation = setup();
        let current_users = 100;

        let new_users = simulation.simulate_adoption(current_users).unwrap();
        assert_eq!(new_users, 100);
    }

    #[test]
    fn test_simulate_adoption_without_rate() {
        let simulation = setup();
        let current_users = 100;

        let new_users = simulation.simulate_adoption(current_users).unwrap();
        assert_eq!(new_users, 100);
    }
}
