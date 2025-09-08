use crate::config::AppConfig;
use crate::error::AppError;
use crate::jwt::verify_keycloak_jwt;
use warp::Filter;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub sub: String,
    pub preferred_username: String,
    pub scope: String,
}

pub fn auth_middleware(
    config: AppConfig,
) -> impl Filter<Extract = (AuthUser,), Error = warp::Rejection> + Clone {
    warp::header::<String>("authorization").and_then(move |auth_header: String| {
        let config = config.clone();
        async move {
            // Extract Bearer token
            if !auth_header.starts_with("Bearer ") {
                return Err(warp::reject::custom(AppError::Unauthorized));
            }

            let token = &auth_header[7..]; // Remove "Bearer " prefix

            // Verify the JWT token
            match verify_keycloak_jwt(token, &config) {
                Ok(claims) => Ok(AuthUser {
                    sub: claims.sub,
                    preferred_username: claims.preferred_username,
                    scope: claims.scope,
                }),
                Err(_) => Err(warp::reject::custom(AppError::Unauthorized)),
            }
        }
    })
}
