use criterion::{criterion_group, criterion_main, Criterion};
use tokenomics_simulator::{Simulation, SimulationOptionsBuilder};

fn benchmark_small_simulation(c: &mut Criterion) {
    // Build a new token
    let token = Simulation::token_builder()
        .name("Token".to_string())
        .symbol("TKN".to_string())
        .total_supply(1_000_000)
        .airdrop_percentage(5.0)
        .burn_rate(1.0)
        .build()
        .unwrap();

    // Build the simulation options
    let options = SimulationOptionsBuilder::new()
        .total_users(100)
        .market_volatility(0.5)
        .build()
        .unwrap();

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Small Simulation".to_string())
        .description("Simulation with 100 users".to_string())
        .token(token)
        .options(options)
        .build()
        .unwrap();

    c.bench_function("run_small_simulation", |b| {
        b.iter(|| {
            simulation.run().unwrap();
        })
    });
}

fn benchmark_large_simulation(c: &mut Criterion) {
    // Build a new token
    let token = Simulation::token_builder()
        .name("LargeToken".to_string())
        .symbol("LTK".to_string())
        .total_supply(1_000_000_000)
        .airdrop_percentage(10.0)
        .burn_rate(2.0)
        .build()
        .unwrap();

    // Build the simulation options
    let options = SimulationOptionsBuilder::new()
        .total_users(500_000)
        .market_volatility(0.8)
        .build()
        .unwrap();

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Large Simulation".to_string())
        .description("Simulation with 1,000,000 users".to_string())
        .token(token)
        .options(options)
        .build()
        .unwrap();

    c.bench_function("run_large_simulation", |b| {
        b.iter(|| {
            simulation.run().unwrap();
        })
    });
}

fn benchmark_extreme_simulation(c: &mut Criterion) {
    // Build a new token
    let token = Simulation::token_builder()
        .name("ExtremeToken".to_string())
        .symbol("ETK".to_string())
        .total_supply(10_000_000_000)
        .airdrop_percentage(20.0)
        .burn_rate(5.0)
        .build()
        .unwrap();

    // Build the simulation options
    let options = SimulationOptionsBuilder::new()
        .total_users(1_000_000)
        .market_volatility(1.0)
        .build()
        .unwrap();

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Extreme Simulation".to_string())
        .description("Simulation with 10,000,000 users".to_string())
        .token(token)
        .options(options)
        .build()
        .unwrap();

    c.bench_function("run_extreme_simulation", |b| {
        b.iter(|| {
            simulation.run().unwrap();
        })
    });
}

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(std::time::Duration::new(300, 0))
}

criterion_group! {
  name = benches;
  config = configure_criterion();
  targets = benchmark_small_simulation, benchmark_large_simulation, benchmark_extreme_simulation
}
criterion_main!(benches);
