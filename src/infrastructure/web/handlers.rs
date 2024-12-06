use axum::{response::Json, extract::State, http::StatusCode};
use serde_json::json;
use crate::infrastructure::database::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

pub async fn home() -> Json<&'static str> {
    Json("Welcome to the Movie Library!")
}

pub async fn list_movies(State(state): State<AppState>) -> Json<serde_json::Value> {
    // Exemplo de consulta ao banco de dados (simulação)
    let movies = vec!["Movie 1", "Movie 2", "Movie 3"];
    Json(json!({ "movies": movies }))
}

pub async fn add_movie() -> StatusCode {
    // Implementar lógica para adicionar filme
    StatusCode::CREATED
}
