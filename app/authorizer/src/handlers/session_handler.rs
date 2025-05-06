use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::TypedHeader;
use headers::Cookie;
use serde_json::json;
use uuid::Uuid;

use crate::{config::AppConfig, services::session_service, state::AppState};

pub async fn refresh_handler(
    State(state): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Response {
    let config = &state.config;
    let db = &state.db_conn;

    let session_cookie = cookie.get("refresh_token");
    if session_cookie.is_none() {
        return (StatusCode::UNAUTHORIZED, "Missing refresh token").into_response();
    }

    let session_id = match Uuid::parse_str(session_cookie.unwrap()) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid session token").into_response(),
    };

    match session_service::validate_and_refresh(db, config, session_id).await {
        Ok(access_token) => Json(json!({ "access_token": access_token })).into_response(),
        Err(_) => (StatusCode::UNAUTHORIZED, "Session expired or invalid").into_response(),
    }
}

pub async fn logout_handler(
    State(state): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Response {
    let db = &state.db_conn;

    let session_cookie = cookie.get("refresh_token");
    if session_cookie.is_none() {
        return (StatusCode::BAD_REQUEST, "Missing refresh token").into_response();
    }

    let session_id = match Uuid::parse_str(session_cookie.unwrap()) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid session token").into_response(),
    };

    if let Err(_) = session_service::revoke_session(db, session_id).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to revoke session",
        )
            .into_response();
    }

    // Clear the cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        format!("refresh_token=deleted; Max-Age=0; Path=/; HttpOnly; Secure; SameSite=Strict")
            .parse()
            .unwrap(),
    );

    (StatusCode::OK, headers, "Logged out").into_response()
}
