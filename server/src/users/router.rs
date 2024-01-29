use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    let router = Router::new();

    router
}
