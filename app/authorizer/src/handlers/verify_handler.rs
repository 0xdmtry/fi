use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use validator::Validate;

use crate::models::verify::VerifyPasscodeRequest;
use crate::services::passcode_service::verify_passcode;
use crate::state::AppState;
use crate::utils::normalize::normalize_email;

pub async fn verify_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<VerifyPasscodeRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let normalized_email = normalize_email(&payload.email);

    verify_passcode(
        &app_state.db_conn,
        &app_state.config,
        &normalized_email,
        &payload.passcode,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Verification failed: {e}"),
        )
    })?;

    Ok(StatusCode::OK)
}
