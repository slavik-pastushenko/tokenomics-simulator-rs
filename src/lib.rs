#![forbid(unsafe_code)]

pub mod engine;
pub mod engine_builder;
pub mod engine_config;
pub mod report;
pub mod token;
pub mod user;

pub use engine::*;
pub use engine_builder::*;
pub use engine_config::*;
pub use report::*;
use thiserror::Error;
pub use token::*;
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
