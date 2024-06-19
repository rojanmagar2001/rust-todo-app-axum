use sqlx::PgPool;

use crate::{
    common::password_encoder,
    user::{models::User, repository as user_repository, views::NewUserRequest},
};

pub async fn register_user(
    pool: PgPool,
    new_user_request: NewUserRequest,
) -> Result<User, sqlx::Error> {
    let new_user_request = NewUserRequest {
        email: new_user_request.email.to_lowercase(),
        password: password_encoder::encode_password(new_user_request.password.as_str()),
        ..new_user_request
    };

    user_repository::register_user(pool, new_user_request).await
}
