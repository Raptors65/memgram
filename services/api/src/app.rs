use axum::Router;

pub fn create_app() -> Router {
    Router::new().merge(crate::routes::health::routes())
}
