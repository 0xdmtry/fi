use crate::payloads::SignupRequest;
use crate::services::wallet_service::create_wallet_service;
use crate::state::AppState;
use axum::http::StatusCode;
use axum::{Json, debug_handler, extract::State};
use validator::Validate;

#[debug_handler]
pub async fn signup_wallet_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation failed: {e}")));
    }

    let _result = create_wallet_service(
        &app_state.db_conn,
        &app_state.config,
        payload.user_id,
        &payload.passcode,
    )
    .await
    .map_err(|err| {
        tracing::error!("create_wallet_service failed: {:?}", err);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Wallet creation failed".into(),
        )
    })?;

    Ok(StatusCode::OK)
}
