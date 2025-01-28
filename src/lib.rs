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
pub use token::*;
pub use user::*;

/// Precision of the decimal numbers used in the simulation.
const DECIMAL_PRECISION: u32 = 4;
