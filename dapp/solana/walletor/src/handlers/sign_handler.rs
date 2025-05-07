use axum::{Json, debug_handler, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::{
    payloads::sign_payload::{SignTransactionRequest, SignTransactionResponse},
    services::sign_service::{SignTransactionArgs, sign_transaction_service},
    state::AppState,
};
use base64;
use validator::Validate;

#[debug_handler]
pub async fn sign_transaction_handler(
    State(state): State<AppState>,
    Json(payload): Json<SignTransactionRequest>,
) -> Result<Json<SignTransactionResponse>, (StatusCode, String)> {
    // Validate payload
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Invalid payload: {e}")));
    }

    // Decode base64 message
    let tx_message_bytes = match base64::decode(&payload.transaction_base64) {
        Ok(bytes) => bytes,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid base64 transaction".into())),
    };

    // Prepare args for service
    let args = SignTransactionArgs {
        user_id: payload.user_id,
        wallet_id: payload.wallet_id,
        transaction_message_bytes: tx_message_bytes,
    };

    // Call the signing service
    let result = sign_transaction_service(&state.db_conn, &state.config, args)
        .await
        .map_err(|err| {
            tracing::error!("Signing failed: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Signing failed".to_string(),
            )
        })?;

    // Prepare response
    let response = SignTransactionResponse {
        signed_transaction_base64: base64::encode(result.signed_transaction_bytes),
        signature: base64::encode(result.signature_bytes),
        wallet_address: Some(result.wallet_address),
        wallet_public_key: Some(result.wallet_public_key),
    };

    Ok(Json(response))
}
