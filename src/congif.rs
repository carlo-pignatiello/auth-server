use std::default;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub issuer: String,
    pub jwt_private_key_path: String,
    pub jwt_public_key_path: String,
    pub jwt_kid: String,
    pub access_token_expiry_secs: i64,
    pub refresh_token_expiry_secs: i64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            database_url:              require("DATABASE_URL")?,
            server_port:               opt("SERVER_PORT", "3000").parse()?,
            issuer:                    require("ISSUER")?,
            jwt_private_key_path:      require("JWT_PRIVATE_KEY_PATH")?,
            jwt_public_key_path:       require("JWT_PUBLIC_KEY_PATH")?,
            jwt_kid:                   require("JWT_KID")?,
            access_token_expiry_secs:  opt("ACCESS_TOKEN_EXPIRY_SECS", "3600").parse()?,
            refresh_token_expiry_secs: opt("REFRESH_TOKEN_EXPIRY_SECS", "2592000").parse()?,
        })  
    }
}

fn require(k: &str) -> anyhow::Result<String> {
    std::env::var(k).map_err(|_| anyhow::anyhow!("Missing env var:  {k}"))
}

fn opt(k: &str, default: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| default.into())
}
