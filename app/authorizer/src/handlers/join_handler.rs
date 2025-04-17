use axum::{debug_handler, extract::{State, Json}, http::StatusCode};
use validator::Validate;
use crate::models::join::JoinRequest;
use crate::services::user_service::process_join_request;
use crate::utils::normalize::normalize_email;
use crate::state::AppState;

#[debug_handler]
pub async fn join_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<JoinRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation failed: {e}")));
    }

    let normalized_email = normalize_email(&payload.email);

    process_join_request(&app_state.db_conn, &app_state.config, &normalized_email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {e}")))?;

    Ok(StatusCode::OK)
}