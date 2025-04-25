use crate::models::passcode::{
    SendFailedPasscodeRequest, SendPasscodeRequest, SendSuccessPasscodeRequest,
};
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use validator::Validate;

pub async fn send_passcode_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SendPasscodeRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    app_state
        .emailer
        .send_and_save_passcode_email(
            &app_state.config,
            &app_state.db_conn,
            &payload.email,
            &payload.passcode,
        )
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Send error: {}", e),
            )
        })?;

    Ok(StatusCode::OK)
}

pub async fn send_success_passcode_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SendSuccessPasscodeRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validate error: {}", e)));
    }

    app_state
        .emailer
        .send_and_save_success_passcode_email(&app_state.config, &app_state.db_conn, &payload.email)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Send error: {}", e),
            )
        })?;

    Ok(StatusCode::OK)
}

pub async fn send_failed_passcode_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SendFailedPasscodeRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validate error: {}", e)));
    }

    app_state
        .emailer
        .send_and_save_failed_passcode_email(&app_state.config, &app_state.db_conn, &payload.email)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Send error: {}", e),
            )
        })?;

    Ok(StatusCode::OK)
}
