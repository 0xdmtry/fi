use crate::{
    payloads::fund_payload::FundWsolRequest, services::fund_service::fund_wsol_ata, state::AppState,
};
use axum::http::StatusCode;
use axum::{Json, extract::State};
use validator::Validate;

pub async fn fund_wsol_handler(
    State(state): State<AppState>,
    Json(payload): Json<FundWsolRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation failed: {e}")));
    }

    let result = fund_wsol_ata(&state.db_conn, &state.config, payload)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Funding failed: {e}"),
            )
        })?;

    Ok(StatusCode::OK)
}
