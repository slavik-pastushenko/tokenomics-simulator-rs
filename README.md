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

## Options

| Option                 | Description                                                               |
|------------------------|---------------------------------------------------------------------------|
| `duration`             | Duration of the simulation, depending on the interval type.               |
| `total_users`          | Number of users in the simulation.                                        |
| `market_volatility`    | Volatility level. 0.0 is no volatility, 1.0 is maximum volatility.        |
| `interval_type`        | Interval type for the simulation (daily or weekly).                       |
| `transaction_fee`      | Transaction fee for each trade.                                           |

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Documentation

For more in-depth details, please refer to the full [documentation](https://docs.rs/tokenomics-simulator).

If you encounter any issues or have questions that are not addressed in the documentation, feel free to [submit an issue](https://github.com/simetrics-io/tokenomics-simulator-rs/issues).

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
