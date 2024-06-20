use axum::{Extension, Json};

use crate::common::error::ApiError;
use crate::common::extractor::JsonOrForm;
use crate::common::middleware::ApiContext;
use crate::user::views::NewUserRequest;

use crate::user::{service as user_service, views::UserView};

pub async fn register_user(
    Extension(ctx): Extension<ApiContext>,
    JsonOrForm(request): JsonOrForm<NewUserRequest>,
) -> Result<Json<UserView>, ApiError> {
    let user = user_service::register_user(ctx.pool, request).await;

    match user {
        Ok(user) => Ok(Json(UserView::from(user))),
        Err(err) => match err {
            sqlx::Error::Database(db_err) => {
                if db_err.constraint().is_some() {
                    Err(ApiError::new_conflict("User already exists".to_string()))
                } else {
                    Err(ApiError::new_internal(db_err.to_string()))
                }
            }
            _ => Err(ApiError::new_internal(err.to_string())),
        },
    }
}
