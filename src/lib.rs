#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! # Tokenomics Simulator
//!
//!
//! An open-source engine for simulating the tokenomics of a project.
//!
//! It allows users to simulate trades, calculate various metrics, and predict user behaviour over different time intervals.
//!
//! <p align="center">
//! <a href="https://simetrics.io" target="_blank" rel="noreferrer noopener">
//! <picture><img src="https://github.com/user-attachments/assets/ac773293-3c63-4300-a0ec-11b213a4ad7b" alt="An open-source engine for simulating the tokenomics of a project." style="width:auto;"></picture>
//! </a>
//! </p>
//!
//! ## Examples
//!
//! In addition to the usage example below, there are more examples available in the [`examples`](https://github.com/simetrics-io/tokenomics-simulator-rs/tree/main/examples) directory of the repository.
//!
//! These examples demonstrate various ways to use the `tokenomics-simulator` crate.
//!
//! ## Usage
//!
//! To use the `tokenomics-simulator` crate in your project, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tokenomics-simulator = "0.5.3"
//! ```
//!
//! You can also include additional features, such as logging, by specifying them in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tokenomics-simulator = { version = "0.5.3", features = ["log", "serde"] }
//! ```
//!
//! Below is an example of how to create and run a simulation using the crate.
//! This example demonstrates how to build simulation options, create a simulation, and run it with a token.
//!
//! ```rust
//! use tokenomics_simulator::{Simulation, SimulationError};
//!
//! fn main() -> Result<(), SimulationError> {
//!     // Build a new token
//!     let token = Simulation::token_builder()
//!         .name("Token".to_string())
//!         .symbol("TKN".to_string())
//!         .total_supply(1_000_000)
//!         .airdrop_percentage(5.0)
//!         .burn_rate(1.0)
//!         .build()?;
//!
//!     // Build the simulation options
//!     let options = Simulation::options_builder()
//!         .total_users(100)
//!         .market_volatility(0.5)
//!         .build()?;
//!
//!     // Build a new simulation with the token and options
//!     let mut simulation = Simulation::builder()
//!         .name("Simulation".to_string())
//!         .description("Initial simulation".to_string())
//!         .token(token)
//!         .options(options)
//!         .build()?;
//!
//!     // Run the simulation
//!     simulation.run()?;
//!
//!     // Get the simulation interval reports
//!     for report in simulation.interval_reports.iter() {
//!         println!("Interval report: {:#?}", report);
//!     }
//!
//!     // Get the final simulation report
//!     println!("Final report: {:#?}", simulation.report);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Benchmarks
//!
//! We use the `criterion` crate to benchmark the performance of the `tokenomics-simulator` crate.
//!
//! Benchmarks help us understand the performance characteristics of the simulation engine and identify areas for optimization.
//!
//! ### Running Benchmarks
//!
//! To run the benchmarks, use the following command:
//!
//! ```sh
//! cargo bench
//! ```
//!
//! ### Benchmark Scenarios
//!
//! We have several benchmark scenarios to test different aspects of the simulation engine:
//!
//! - **Small Simulation**: Tests a simulation with 100 users.
//! - **Large Simulation**: Tests a simulation with 500,000 users.
//! - **Extreme Simulation**: Tests a simulation with 1,000,000 users.
//!
//! For more detailed benchmark scenarios, please refer to the [`benches`](https://github.com/simetrics-io/tokenomics-simulator-rs/tree/main/benches) directory in the repository.
//!
//! ## Safety
//!
//! This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.
//!
//! ## Contributing
//!
//! 🎈 Thanks for your help improving the project! We are so happy to have you!
//!
//! We have a [contributing guide](https://github.com/simetrics-io/tokenomics-simulator-rs/blob/main/CONTRIBUTING.md) to help you get involved in the project.

use thiserror::Error;

/// Engine module.
/// Is used to run the simulation with the desired configuration.
pub mod engine;

/// Engine builder module.
/// Is used to create a new engine with the desired configuration.
pub mod engine_builder;

/// Engine configuration module.
/// Is used to create a new engine configuration.
pub mod engine_config;

/// Report module.
/// Is used to generate reports.
pub mod report;

/// Token module.
/// Is used to apply token related operations for the simulation.
pub mod token;

/// Token builder module.
/// Is used to create a new token with the desired configuration.
pub mod token_builder;

/// User module.
/// Is used to apply user related operations for the simulation.
pub mod user;

pub use engine::*;
pub use engine_builder::*;
pub use engine_config::*;
pub use report::*;
pub use token::*;
pub use token_builder::*;
pub use user::*;

/// Simulation error.
/// A list of possible errors that can occur during the simulation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SimulationError {
    /// Missing required field: name.
    #[error("Missing required field: name.")]
    MissingName,

    /// Missing required field: token.
    #[error("Missing required field: token.")]
    MissingToken,

    /// Missing required field: options.
    #[error("Missing required field: options.")]
    MissingOptions,

    /// Missing required field: total_users.
    #[error("Missing required field: total_users.")]
    MissingTotalUsers,

    /// Invalid decimal value.
    #[error("Invalid decimal value.")]
    InvalidDecimal,
}
