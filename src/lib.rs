#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Tokenomics Simulator
//!
//! An open-source engine for simulating the tokenomics of a project.

use thiserror::Error;

/// Engine module.
pub mod engine;

/// Engine builder module.
pub mod engine_builder;

/// Engine configuration module.
pub mod engine_config;

/// Report module.
pub mod report;

/// Token module.
pub mod token;

/// Token builder module.
pub mod token_builder;

/// User module.
pub mod user;

pub use engine::*;
pub use engine_builder::*;
pub use engine_config::*;
pub use report::*;
pub use token::*;
pub use token_builder::*;
pub use user::*;

/// Precision of the decimal numbers used in the simulation.
const DECIMAL_PRECISION: u32 = 4;

/// Simulation error.
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
