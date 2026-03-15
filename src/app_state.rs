use std::sync::Arc;
use sqlx::PgPool;
use crate::jwt::JwtKeys;
use crate::congif::Config;

#[derive(Clone)]
pub struct AppState {
    pub config:   Arc<Config>,
    pub pool:     PgPool,
    pub jwt_keys: Arc<JwtKeys>,
}
