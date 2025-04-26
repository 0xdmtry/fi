use axum::{
    debug_handler,
    extract::{Json, State},
    http::StatusCode,
};
use validator::Validate;

use crate::models::resend::ResendRequest;
use crate::services::passcode_service::resend_passcode;
use crate::state::AppState;
use crate::utils::normalize::normalize_email;

#[debug_handler]
pub async fn resend_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<ResendRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation failed: {e}")));
    }

    let normalized_email = normalize_email(&payload.email);

    resend_passcode(&app_state.db_conn, &app_state.config, &normalized_email)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {e}"),
            )
        })?;

    Ok(StatusCode::OK)
}
