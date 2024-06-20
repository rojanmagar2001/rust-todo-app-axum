use crate::user::handlers::register_user;
use axum::{routing::post, Router};

pub fn route() -> Router {
    Router::new().route("/user", post(register_user))
}
