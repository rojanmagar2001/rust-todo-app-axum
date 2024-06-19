use std::sync::Arc;

use axum::Router;
use rust_axum_todo_api::{
    common::middleware::ApiState, config::Config, user::routes as user_routes,
};
use sqlx::{Pool, Postgres};

pub fn build_routes(pool: Pool<Postgres>, config: Config) -> Router {
    let api_state = ApiState {
        pool,
        config: Arc::new(config),
    };

    let api_routes = Router::new()
        .nest("/users", user_routes::route())
        .with_state(api_state);

    Router::new().nest("/api", api_routes)
}
