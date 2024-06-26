use std::sync::Arc;

use crate::config::Config;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct ApiContext {
    pub pool: Pool<Postgres>,
    pub config: Arc<Config>,
}
