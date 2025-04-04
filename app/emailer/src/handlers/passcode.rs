use axum::{extract::Extension, Json};
use axum::http::StatusCode;
use crate::models::passcode::SendPasscodeRequest;
use validator::Validate;
use std::sync::Arc;
use crate::services::Emailer;

pub async fn send_passcode_handler(
                                    Extension(emailer): Extension<Arc<dyn Emailer>>,
                                    Json(payload): Json<SendPasscodeRequest>,
                                ) -> Result<StatusCode, (StatusCode, String)> {

    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    emailer
        .send_passcode(&payload.email, &payload.passcode)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Send error: {}", e)))?;

    Ok(StatusCode::OK)
}