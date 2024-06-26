use sqlx::PgPool;

use super::{models::User, views::NewUserRequest};

pub async fn register_user(
    pool: PgPool,
    new_user_request: NewUserRequest,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password, nickname)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(new_user_request.email)
    .bind(new_user_request.password)
    .bind(new_user_request.nickname)
    .fetch_one(&pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(pool: PgPool, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users
        WHERE email = $1 AND
        deleted_at IS NULL
        "#,
    )
    .bind(email)
    .fetch_one(&pool)
    .await?;

    Ok(user)
}
