# Tokenomics Simulator CLI Example

This example demonstrates how to use the `tokenomics-simulator` crate to create and run a tokenomics simulation via a command-line interface (CLI).

## Documentation

For more in-depth details, please refer to the full [documentation](https://docs.rs/tokenomics-simulator).

If you encounter any issues or have questions that are not addressed in the documentation, feel free to [submit an issue](https://github.com/simetrics-io/tokenomics-simulator-rs/issues).

## Usage

To run the CLI example, use the following command:

```sh
cargo run --example cli -- \
    --name "Token" \
    --symbol "TKN" \
    --total_supply 1000000 \
    --airdrop_percentage 5.0 \
    --burn_rate 1.0 \
    --total_users 100 \
    --market_volatility 0.5
```

This command will:

- Create a token named "Token" with the symbol "TKN".
- Set the total supply to 1,000,000.
- Set the airdrop percentage to 5.0%.
- Set the burn rate to 1.0%.
- Set the total number of users for the simulation to 100.
- Set the market volatility to 0.5.
- Run the simulation and a final report will be printed to stdout.

## Help

For more details on how to use the CLI and the available options, run:

```sh
cargo run --example cli -- --help
```

This will display the help message with descriptions of all the available command-line arguments:

```sh
Usage: cli --name <NAME> --symbol <SYMBOL> --total_supply <TOTAL_SUPPLY> --airdrop_percentage <AIRDROP_PERCENTAGE> --burn_rate <BURN_RATE> --total_users <TOTAL_USERS> --market_volatility <MARKET_VOLATILITY>

Options:
  -n, --name <NAME>                              Set the name of the token
  -s, --symbol <SYMBOL>                          Set the symbol of the token
  -t, --total_supply <TOTAL_SUPPLY>              Set the total supply of the token
  -a, --airdrop_percentage <AIRDROP_PERCENTAGE>  Set the airdrop percentage of the token
  -b, --burn_rate <BURN_RATE>                    Set the burn rate of the token
  -u, --total_users <TOTAL_USERS>                Set the total number of users for the simulation
  -v, --market_volatility <MARKET_VOLATILITY>    Set the market volatility for the simulation
  -h, --help                                     Print help
  -V, --version                                  Print version
```