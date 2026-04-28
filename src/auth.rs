use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use base64::{engine::general_purpose, Engine as _};
use crate::app_state::AppState;

pub async fn basic_auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let config = &state.config;

    // Only enforce auth if both username and password are set
    if let (Some(username), Some(password)) = (&config.app_username, &config.app_password) {
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|val| val.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Basic ") {
                let encoded = &auth_header[6..];
                if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
                    if let Ok(decoded_str) = String::from_utf8(decoded) {
                        if let Some((u, p)) = decoded_str.split_once(':') {
                            if u == username && p == password {
                                return Ok(next.run(req).await);
                            }
                        }
                    }
                }
            }
        }

        // Authentication failed or missing
        tracing::warn!("Unauthorized access attempt to {}", req.uri());
        return Err(StatusCode::UNAUTHORIZED);
    }

    // No authentication configured
    Ok(next.run(req).await)
}
