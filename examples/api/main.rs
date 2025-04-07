use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tokio::net::TcpListener;

mod simulation;
mod validator;

/// Exceptions that can be thrown by the application.
#[derive(Debug, Error, Serialize, PartialEq, Eq)]
pub enum Exception {
    /// Token not found.
    #[serde(rename = "TOKEN_NOT_FOUND")]
    #[error("Token not found.")]
    TokenNotFound,

    /// Simulation not found.
    #[serde(rename = "SIMULATION_NOT_FOUND")]
    #[error("Simulation not found.")]
    SimulationNotFound,

    /// Internal error.
    #[serde(rename = "INTERNAL_ERROR")]
    #[error("Internal error.")]
    InternalError,

    /// Invalid input.
    #[serde(rename = "INVALID_INPUT")]
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Validation failed.
    #[serde(rename = "VALIDATION_FAILED")]
    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    /// Payload too large.
    #[serde(rename = "PAYLOAD_TOO_LARGE")]
    #[error("Payload too large.")]
    PayloadTooLarge,
}

impl IntoResponse for Exception {
    /// Convert the `Exception` into an HTTP response.
    ///
    /// # Returns
    ///
    /// `Response` containing the appropriate status code and JSON representation of the exception.
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": &self.to_string() })),
        )
            .into_response()
    }
}

/// Create the application.
///
/// # Returns
///
/// The application router.
pub async fn app() -> Router {
    Router::new().route("/simulation", post(simulation::create))
}

#[tokio::main]
async fn main() {
    let address_app = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server is running on {}", address_app);

    let listener = TcpListener::bind(&address_app).await.unwrap();
    axum::serve(listener, app().await.into_make_service())
        .await
        .unwrap();
}
