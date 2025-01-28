# Tokenomics Simulator

Tokenomics Simulator is a tool for simulating the tokenomics of a project.

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

## Usage

To use the `tokenomics-simulator` crate in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
tokenomics-simulator = "x.x.x"
```

Below is an example of how to create and run a simulation using the crate.
This example demonstrates how to build simulation options, create a simulation, and run it with a token.
For more detailed information and advanced usage, please refer to the full [documentation](https://docs.rs/tokenomics-simulator).

```rust
use tokenomics_simulator::{Simulation, SimulationError, Token};

fn main() -> Result<(), SimulationError> {
    // Build the simulation options
    let options = Simulation::options_builder()
        .total_users(100)
        .market_volatility(0.5)
        .build()?;

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Simulation".to_string())
        .description("Initial simulation".to_string())
        .token(Token::default())
        .options(options)
        .build()?;

    // Run the simulation
    if let Err(error) = simulation.run() {
        return Err(error);
    }

    // Get the simulation interval reports
    for (time, report) in simulation.interval_reports.iter() {
        println!("Interval {}: {:#?}", time, report);
    }

    // Get the final simulation report
    println!("Final report: {:#?}", simulation.report);

    Ok(())
}
```

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Contributing

Contributions from the community are always welcome! Here are some ways you can contribute:

### Reporting Bugs

If you encounter any bugs, please [submit an issue](https://github.com/simetrics-io/tokenomics-simulator-rs/issues) with detailed information about the problem and steps to reproduce it.

### Feature Requests

If you have ideas for new features, feel free to [submit an issue](https://github.com/simetrics-io/tokenomics-simulator-rs/issues) with a detailed description of the feature and its potential use cases.

### Build

To build the project, run:

```bash
cargo build
```

### Test

To run the tests, use:

```bash
cargo test
```

### Lint

Run [clippy](https://github.com/rust-lang/rust-clippy) to lint the code:

```bash
cargo clippy --all-targets --all-features --no-deps -- -D warnings
```

### Format

Run [rustfmt](https://github.com/rust-lang/rustfmt) to format the code:

```bash
cargo fmt
```

### Documentation

Generate documentation in HTML format:

```bash
cargo doc --open
```
