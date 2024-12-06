use crate::infrastructure::web::handlers;
use crate::infrastructure::web::handlers::AppState;
use axum::{routing::get, Router};

pub fn create_routes() -> Router<AppState> {
    Router::new().route("/", get(handlers::home)).route(
        "/movies",
        get(handlers::list_movies).post(handlers::add_movie),
    )
}
