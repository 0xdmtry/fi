use crate::payloads::{SignupRequest, SignupResponse};
use crate::services::wallet_service::create_wallet_service;
use crate::state::AppState;
use axum::http::StatusCode;
use axum::{Json, debug_handler, extract::State};
use sea_orm::DatabaseConnection;

#[debug_handler]
pub async fn signup_wallet_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = create_wallet_service(
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
