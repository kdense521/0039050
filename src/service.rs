use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn app_router() -> Router {
    let app_state = App::default();

    Router::new()
        .route("/movie/:id", get(get_movie))
        .route("/movie", post(add_movie))
        .with_state(app_state)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub id: String,
    pub name: String,
    pub year: u16,
    pub was_good: bool,
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub movies: HashMap<String, Movie>,
}

pub type App = Arc<Mutex<AppState>>;

async fn add_movie(State(state): State<App>, Json(movie): Json<Movie>) {
    let mut state = state.lock().await;

    state.movies.insert(movie.id.clone(), movie);
}

async fn get_movie(State(state): State<App>, Path(movie_id): Path<String>) -> Json<Option<Movie>> {
    let state = state.lock().await;

    Json(state.movies.get(&movie_id).cloned())
}
