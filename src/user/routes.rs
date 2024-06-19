use crate::user::handlers::register_user;
use axum::{routing::post, Router};

pub fn route() -> Router {
    Router::new().route("/", post(register_user))
}
