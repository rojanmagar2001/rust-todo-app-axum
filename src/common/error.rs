use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
// use sqlx::error::DatabaseError;
use thiserror::Error;
use tracing::error;
use utoipa::ToSchema;

// #[derive(thiserror::Error, Debug)]
// pub enum Error {
//     #[error("Internal Server Error")]
//     InternalServerError,
//     #[error("Not Found")]
//     NotFound,
//     #[error("Unauthorized")]
//     Unauthorized,
//     #[error("Forbidden")]
//     Forbidden,
//     #[error("Bad Request")]
//     BadRequest,
//     #[error("Conflict")]
//     Conflict,
//     #[error("Unprocessable Entity")]
//     UnprocessableEntity,
// }

#[derive(Serialize, Deserialize, Error, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    status_code: u16,
    errors: Vec<String>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Err {}", &self.status_code)
    }
}

impl ApiError {
    pub fn new(status_code: u16, error: String) -> Self {
        Self {
            status_code,
            errors: vec![error],
        }
    }

    pub fn new_internal(error: String) -> Self {
        error!("{}", error);
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            errors: vec!["Internal Server Error".to_string()],
        }
    }

    pub fn new_bad_request(error: String) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            errors: vec![error],
        }
    }

    pub fn new_not_found(error: String) -> Self {
        Self {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            errors: vec![error],
        }
    }

    pub fn new_unauthorized(error: String) -> Self {
        Self {
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
            errors: vec![error],
        }
    }

    pub fn new_forbidden(error: String) -> Self {
        Self {
            status_code: StatusCode::FORBIDDEN.as_u16(),
            errors: vec![error],
        }
    }

    pub fn new_conflict(error: String) -> Self {
        Self {
            status_code: StatusCode::CONFLICT.as_u16(),
            errors: vec![error],
        }
    }

    pub fn append_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn new_unprocessable_entity(error: String) -> Self {
        Self {
            status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            errors: vec![error],
        }
    }

    pub fn new_validation_error(errors: Vec<String>) -> Self {
        Self {
            status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            errors,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap();
        axum::http::Response::builder()
            .status(self.status_code)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}

// pub trait ResultExt<T> {
//     /// If `self` contains a SQLx database constraint error with the given name,
//     /// transform the error.
//     ///
//     /// Otherwise, the result is passed through unchanged.
//     fn on_constraint(
//         self,
//         name: &str,
//         f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
//     ) -> Result<T, Error>;
// }
