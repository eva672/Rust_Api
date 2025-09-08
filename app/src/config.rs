use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // Application settings
    pub app_host: String,
    pub app_port: u16,

    // Database
    pub database_url: String,

    // Keycloak settings
    pub keycloak_base_url: String,
    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(AppConfig {
            app_host: std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            app_port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("Invalid APP_PORT")?,
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            keycloak_base_url: std::env::var("KEYCLOAK_BASE_URL")
                .context("KEYCLOAK_BASE_URL must be set")?,
            keycloak_realm: std::env::var("KEYCLOAK_REALM")
                .context("KEYCLOAK_REALM must be set")?,
            keycloak_client_id: std::env::var("KEYCLOAK_CLIENT_ID")
                .context("KEYCLOAK_CLIENT_ID must be set")?,
            keycloak_client_secret: std::env::var("KEYCLOAK_CLIENT_SECRET").ok(),
        })
    }

    pub fn realm_issuer_url(&self) -> String {
        format!("{}/realms/{}", self.keycloak_base_url, self.keycloak_realm)
    }

    pub fn jwks_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/certs",
            self.keycloak_base_url, self.keycloak_realm
        )
    }

    pub fn introspect_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token/introspect",
            self.keycloak_base_url, self.keycloak_realm
        )
    }
}
