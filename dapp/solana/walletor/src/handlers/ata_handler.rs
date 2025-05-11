use crate::{
    payloads::ata_payload::CreateAtaRequest, services::ata_service::create_ata_service,
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

pub async fn create_ata_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateAtaRequest>,
) -> Result<StatusCode, (StatusCode, String)> {

    if let Err(e) = payload.validate() {
        println!("create_ata_handler::ERROR: {:?}", e);
        return Err((StatusCode::BAD_REQUEST, format!("Invalid payload: {e}")));
    }

    create_ata_service(&state.db_conn, &state.config, payload)
        .await
        .map_err(|e| {
            tracing::error!("ATA creation failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "ATA creation failed".to_string(),
            )
        })?;

    Ok(StatusCode::OK)
}
