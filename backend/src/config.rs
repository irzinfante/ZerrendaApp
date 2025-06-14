use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub auth_private_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            app_port: env::var("APP_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse::<u16>()
                .expect("APP_PORT must be a number"),

            db_host: env::var("DB_HOST").expect("DB_HOST is required"),
            db_user: env::var("DB_USER").expect("DB_USER is required"),
            db_password: env::var("DB_PASSWORD").expect("DB_PASSWORD is required"),

            auth_private_key: env::var("AUTH_PRIVATE_KEY").expect("AUTH_PRIVATE_KEY is required"),
        }
    }
}
