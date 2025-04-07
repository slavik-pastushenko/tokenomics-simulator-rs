use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use tokenomics_simulator::{Simulation, SimulationBuilder};

use crate::{validator::Validator, Exception};

/// Create a new simulation.
///
/// # Arguments
///
/// * `data` - Simulation input data.
///
/// # Returns
///
/// Created simulation.
pub async fn create(Json(data): Json<SimulationBuilder>) -> impl IntoResponse {
    let token = match data.token {
        Some(token) => match token.validate() {
            Ok(_) => token,
            Err(err) => return err.into_response(),
        },
        _ => return Exception::TokenNotFound.into_response(),
    };

    let mut simulation_builder = Simulation::builder().token(token);

    if let Some(name) = data.name {
        simulation_builder = simulation_builder.name(name);
    }

    if let Some(description) = data.description {
        simulation_builder = simulation_builder.description(description);
    }

    if let Some(options) = data.options {
        if let Err(err) = options.validate() {
            return err.into_response();
        }

        simulation_builder = simulation_builder.options(options);
    }

    let mut simulation = match simulation_builder.build() {
        Ok(simulation) => simulation,
        Err(err) => {
            println!("Failed to build simulation: {:?}", err);
            return Exception::InternalError.into_response();
        }
    };

    match simulation.run() {
        Ok(_) => (StatusCode::CREATED, Json(simulation)).into_response(),
        Err(err) => {
            println!("Failed to run simulation: {:?}", err);
            Exception::InternalError.into_response()
        }
    }
}
