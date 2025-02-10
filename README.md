<p align="center">
    <a href="https://simetrics.io" target="_blank">
          <picture><img src="https://github.com/user-attachments/assets/406ac872-0adc-406b-9372-77dbe53a1346" alt="An open-source engine for simulating the tokenomics of a project." style="width:auto;"></picture>
    </a>
</p>

# Tokenomics Simulator

An open-source engine for simulating the tokenomics of a project.

It allows users to simulate trades, calculate various metrics, and predict user behaviour over different time intervals.

## Reference implementation

[![test](https://github.com/simetrics-io/tokenomics-simulator-rs/actions/workflows/test.yml/badge.svg)](https://github.com/simetrics-io/tokenomics-simulator-rs/actions/workflows/test.yml)
[![docs](https://docs.rs/tokenomics-simulator/badge.svg)](https://docs.rs/tokenomics-simulator)
[![crate](https://img.shields.io/crates/v/tokenomics-simulator.svg)](https://crates.io/crates/tokenomics-simulator)
![downloads](https://img.shields.io/crates/d/tokenomics-simulator)
![GitHub](https://img.shields.io/github/license/simetrics-io/tokenomics-simulator-rs)
[![codecov](https://codecov.io/gh/simetrics-io/tokenomics-simulator-rs/graph/badge.svg?token=4MU5JOXW27)](https://codecov.io/gh/simetrics-io/tokenomics-simulator-rs)

## Documentation

For more in-depth details, please refer to the full [documentation](https://docs.rs/tokenomics-simulator).

If you encounter any issues or have questions that are not addressed in the documentation, feel free to [submit an issue](https://github.com/simetrics-io/tokenomics-simulator-rs/issues).

## Examples

In addition to the usage example below, there are more examples available in the [`examples`](https://github.com/simetrics-io/tokenomics-simulator-rs/tree/main/examples) directory of the repository.

These examples demonstrate various ways to use the `tokenomics-simulator` crate.

## Usage

To use the `tokenomics-simulator` crate in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
tokenomics-simulator = "0.1.31"
```

Below is an example of how to create and run a simulation using the crate.
This example demonstrates how to build simulation options, create a simulation, and run it with a token.
For more detailed information and advanced usage, please refer to the full [documentation](https://docs.rs/tokenomics-simulator).

```rust
use tokenomics_simulator::{Simulation, SimulationError, SimulationTransactionFee};

fn main() -> Result<(), SimulationError> {
    // Build a new token
    let token = Simulation::token_builder()
        .name("Token".to_string())
        .symbol("TKN".to_string())
        .total_supply(1_000_000)
        .airdrop_percentage(5.0)
        .burn_rate(1.0)
        .build()?;

    // Build the simulation options
    let options = Simulation::options_builder()
        .total_users(100)
        .market_volatility(0.5)
        .transaction_fee(SimulationTransactionFee::Custom(0.01))
        .build()?;

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Simulation".to_string())
        .description("Initial simulation".to_string())
        .token(token)
        .options(options)
        .build()?;

    // Run the simulation
    simulation.run()?;

    // Get the simulation interval reports
    for (time, report) in simulation.interval_reports.iter() {
        println!("Interval {}: {:#?}", time, report);
    }

    // Get the final simulation report
    println!("Final report: {:#?}", simulation.report);

    Ok(())
}
```

## Benchmarks

We use the `criterion` crate to benchmark the performance of the `tokenomics-simulator` crate.

Benchmarks help us understand the performance characteristics of the simulation engine and identify areas for optimization.

We have several benchmark scenarios to test different aspects of the simulation engine:

- **Small Simulation**: Tests a simulation with 100 users.
- **Large Simulation**: Tests a simulation with 500,000 users.
- **Extreme Simulation**: Tests a simulation with 1,000,000 users.

For more detailed benchmark scenarios, please refer to the [`benches`](https://github.com/simetrics-io/tokenomics-simulator-rs/tree/main/benches) directory in the repository.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Contributing

🎈 Thanks for your help improving the project! We are so happy to have you!

We have a [contributing guide](https://github.com/simetrics-io/tokenomics-simulator-rs/blob/main/CONTRIBUTING.md) to help you get involved in the project.
