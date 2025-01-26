#![forbid(unsafe_code)]

pub mod engine;
pub mod report;
pub mod token;
pub mod user;

pub use engine::*;
pub use report::*;
pub use token::*;
pub use user::*;

/// Precision of the decimal numbers used in the simulation.
const DECIMAL_PRECISION: u32 = 4;
