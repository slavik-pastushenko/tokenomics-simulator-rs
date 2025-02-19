//! # Engine builder module
//!
//! This module contains the builder for creating a new simulation.
//!
//! The builder allows for configuring the simulation before building it.
//! The builder is used to ensure that all required fields are provided when creating a new simulation.

use chrono::Utc;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    Simulation, SimulationError, SimulationIntervalReports, SimulationOptions, SimulationReport,
    SimulationStatus, Token,
};

/// Builder for creating a new simulation.
#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SimulationBuilder {
    /// Name of the simulation.
    /// Required field.
    pub name: Option<String>,

    /// Token used to run the simulation.
    /// Required field.
    pub token: Option<Token>,

    /// Description of the simulation.
    /// Optional field.
    pub description: Option<String>,

    /// Input parameters for the simulation.
    /// Required field.
    pub options: Option<SimulationOptions>,
}

impl SimulationBuilder {
    /// Create a new simulation builder to configure the simulation.
    ///
    /// # Returns
    ///
    /// New simulation builder.
    pub fn new() -> Self {
        SimulationBuilder::default()
    }

    /// Set the name of the simulation.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the simulation.
    ///
    /// # Returns
    ///
    /// The simulation builder.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the token used to run the simulation.
    ///
    /// # Arguments
    ///
    /// * `token` - Token used to run the simulation.
    ///
    /// # Returns
    ///
    /// The simulation builder.
    pub fn token(mut self, token: Token) -> Self {
        self.token = Some(token);
        self
    }

    /// Set the description of the simulation.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the simulation.
    ///
    /// # Returns
    ///
    /// The simulation builder.
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the input parameters for the simulation.
    ///
    /// # Arguments
    ///
    /// * `options` - Input parameters for the simulation.
    ///
    /// # Returns
    ///
    /// The simulation builder.
    pub fn options(mut self, options: SimulationOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Build the simulation.
    ///
    /// # Returns
    ///
    /// Built simulation or an error if required fields are missing.
    pub fn build(self) -> Result<Simulation, SimulationError> {
        Ok(Simulation {
            id: Uuid::new_v4(),
            description: self.description,
            status: SimulationStatus::Pending,
            name: self.name.ok_or(SimulationError::MissingName)?,
            token: self.token.ok_or(SimulationError::MissingToken)?,
            options: self.options.ok_or(SimulationError::MissingOptions)?,
            interval_reports: SimulationIntervalReports::default(),
            report: SimulationReport::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::{SimulationInterval, TokenBuilder, ValuationModel};

    use super::*;

    #[test]
    fn test_new_simulation_builder() {
        let builder = SimulationBuilder::new();

        assert_eq!(builder.name, None);
        assert_eq!(builder.token, None);
        assert_eq!(builder.description, None);
        assert_eq!(builder.options, None);
    }

    #[test]
    fn test_build_simulation() {
        let token = TokenBuilder::new()
            .name("Test Token".to_string())
            .total_supply(1_000_000)
            .build()
            .unwrap();
        let options = SimulationOptions {
            duration: 30,
            total_users: 100,
            decimal_precision: 4,
            market_volatility: Decimal::new(5, 1),
            transaction_fee: None,
            interval_type: SimulationInterval::Daily,
            adoption_rate: None,
            valuation_model: Some(ValuationModel::Exponential(1.0)),
        };

        let simulation = SimulationBuilder::default()
            .name("Test Simulation".to_string())
            .description("Test Simulation".to_string())
            .token(token.clone())
            .options(options.clone())
            .build()
            .unwrap();

        assert_eq!(simulation.name, "Test Simulation");
        assert_eq!(simulation.token, token);
        assert_eq!(simulation.options, options);
    }

    #[test]
    fn test_build_simulation_missing_name() {
        let token = TokenBuilder::new()
            .name("Test Token".to_string())
            .total_supply(1_000_000)
            .build()
            .unwrap();
        let options = SimulationOptions {
            duration: 30,
            total_users: 100,
            decimal_precision: 4,
            market_volatility: Decimal::new(5, 1),
            transaction_fee: None,
            interval_type: SimulationInterval::Daily,
            adoption_rate: None,
            valuation_model: Some(ValuationModel::Exponential(1.0)),
        };

        let simulation = SimulationBuilder::default()
            .token(token)
            .options(options)
            .build();

        assert!(simulation.is_err());
        assert_eq!(simulation.unwrap_err(), SimulationError::MissingName);
    }

    #[test]
    fn test_build_simulation_missing_token() {
        let options = SimulationOptions {
            duration: 30,
            total_users: 100,
            decimal_precision: 4,
            market_volatility: Decimal::new(5, 1),
            transaction_fee: None,
            interval_type: SimulationInterval::Daily,
            adoption_rate: None,
            valuation_model: Some(ValuationModel::Exponential(1.0)),
        };

        let simulation = SimulationBuilder::default()
            .name("Test Simulation".to_string())
            .options(options)
            .build();

        assert!(simulation.is_err());
        assert_eq!(simulation.unwrap_err(), SimulationError::MissingToken);
    }

    #[test]
    fn test_build_simulation_missing_options() {
        let token = TokenBuilder::new()
            .name("Test Token".to_string())
            .total_supply(1_000_000)
            .build()
            .unwrap();

        let simulation = SimulationBuilder::default()
            .name("Test Simulation".to_string())
            .token(token)
            .build();

        assert!(simulation.is_err());
        assert_eq!(simulation.unwrap_err(), SimulationError::MissingOptions);
    }
}
