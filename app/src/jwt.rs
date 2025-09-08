use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::config::AppConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub preferred_username: String,
    pub exp: u64,
    pub iat: u64,
    pub aud: String,
    pub iss: String,
    pub azp: String,
    pub scope: String,
}

// Global cache for JWKS
lazy_static! {
    static ref JWKS_CACHE: std::sync::RwLock<HashMap<String, String>> =
        std::sync::RwLock::new(HashMap::new());
}

// Fetches the JWKS from Keycloak
fn fetch_jwks(cfg: &AppConfig) -> Result<Value> {
    let response = ureq::get(&cfg.jwks_url()).call()?.into_json::<Value>()?;
    Ok(response)
}

// Populates the JWKS cache
pub fn populate_jwks_cache(cfg: &AppConfig) -> Result<()> {
    let jwks = fetch_jwks(cfg)?;
    let mut cache = JWKS_CACHE.write().unwrap();
    cache.clear();

    if let Some(keys) = jwks.get("keys").and_then(|k| k.as_array()) {
        for k in keys {
            if let (Some(kty), Some(kid)) = (k.get("kty"), k.get("kid")) {
                if kty == "RSA" {
                    if let (Some(n), Some(e)) = (k.get("n"), k.get("e")) {
                        let key_data = format!(
                            "{}:{}",
                            n.as_str().unwrap_or_default(),
                            e.as_str().unwrap_or_default()
                        );
                        cache.insert(kid.as_str().unwrap_or_default().to_string(), key_data);
                    }
                }
            }
        }
    }
    println!("✅ JWKS cache populated successfully");
    Ok(())
}

// Simple base64url decode without external dependencies
fn base64url_decode(input: &str) -> Result<Vec<u8>> {
    let mut input = input.to_string();

    // Add padding if needed
    while input.len() % 4 != 0 {
        input.push('=');
    }

    // Replace URL-safe characters with standard base64
    let input = input.replace('-', "+").replace('_', "/");

    // Simple base64 decode implementation
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < input.len() {
        let mut val = 0u32;
        let mut j = 0;
        
        while j < 4 && i + j < input.len() {
            let c = input.chars().nth(i + j).unwrap_or('=');
            if c == '=' {
                break;
            }
            if let Some(pos) = chars.find(c) {
                val = (val << 6) | (pos as u32);
            }
            j += 1;
        }
        
        // Convert 24-bit value to bytes
        if j >= 2 {
            result.push((val >> 16) as u8);
        }
        if j >= 3 {
            result.push((val >> 8) as u8);
        }
        if j >= 4 {
            result.push(val as u8);
        }
        
        i += 4;
    }
    
    Ok(result)
}

pub fn verify_keycloak_jwt(token: &str, cfg: &AppConfig) -> Result<Claims> {
    // Parse the JWT header to get the kid
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(anyhow::anyhow!("Invalid JWT format"));
    }

    let header_bytes = base64url_decode(parts[0])?;
    let header: Value = serde_json::from_slice(&header_bytes)?;
    let _kid = header
        .get("kid")
        .and_then(|k| k.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing kid in JWT header"))?;

    // Parse the payload
    let payload_bytes = base64url_decode(parts[1])?;
    let payload: Value = serde_json::from_slice(&payload_bytes)?;

    // Basic validation
    let iss = payload
        .get("iss")
        .and_then(|i| i.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing iss in JWT payload"))?;

    if iss != cfg.realm_issuer_url().as_str() {
        return Err(anyhow::anyhow!("Invalid issuer"));
    }

    let aud = payload
        .get("aud")
        .and_then(|a| a.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing aud in JWT payload"))?;

    if aud != cfg.keycloak_client_id {
        return Err(anyhow::anyhow!("Invalid audience"));
    }

    // Check expiration
    let exp = payload
        .get("exp")
        .and_then(|e| e.as_u64())
        .ok_or_else(|| anyhow::anyhow!("Missing exp in JWT payload"))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if exp < now {
        return Err(anyhow::anyhow!("Token expired"));
    }

    // Extract claims
    let claims = Claims {
        sub: payload
            .get("sub")
            .and_then(|s| s.as_str())
            .unwrap_or_default()
            .to_string(),
        preferred_username: payload
            .get("preferred_username")
            .and_then(|u| u.as_str())
            .unwrap_or_default()
            .to_string(),
        exp,
        iat: payload.get("iat").and_then(|i| i.as_u64()).unwrap_or(0),
        aud: aud.to_string(),
        iss: iss.to_string(),
        azp: payload
            .get("azp")
            .and_then(|a| a.as_str())
            .unwrap_or_default()
            .to_string(),
        scope: payload
            .get("scope")
            .and_then(|s| s.as_str())
            .unwrap_or_default()
            .to_string(),
    };

    Ok(claims)
}

// Optional: online token introspection with Keycloak
#[derive(Deserialize)]
struct IntrospectionResponse {
    active: bool,
    sub: Option<String>,
    username: Option<String>,
    exp: Option<u64>,
    iat: Option<u64>,
    iss: Option<String>,
    aud: Option<serde_json::Value>,
    scope: Option<String>,
}

pub fn introspect_token(token: &str, cfg: &AppConfig) -> Result<bool> {
    let mut form: Vec<(&str, &str)> = vec![
        ("token", token),
        ("client_id", &cfg.keycloak_client_id),
    ];

    if let Some(secret) = &cfg.keycloak_client_secret {
        form.push(("client_secret", secret));
    }

    let response = ureq::post(&cfg.introspect_url()).send_form(&form)?;

    if response.status() != 200 {
        return Ok(false);
    }

    let body: IntrospectionResponse = response.into_json()?;
    Ok(body.active)
}
