#![forbid(unsafe_code)]

pub mod engine;
pub mod report;
pub mod token;
pub mod user;
pub mod vesting;

pub use engine::*;
pub use report::*;
pub use token::*;
pub use user::*;
pub use vesting::*;

/// Precision of the decimal numbers used in the simulation.
const DECIMAL_PRECISION: u32 = 4;
