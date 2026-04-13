use crate::handlers::health::health;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/health", get(health))
}
