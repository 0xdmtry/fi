use axum::{Json, extract::Extension};
use axum::http::StatusCode;
use crate::models::join::JoinRequest;
use crate::services::user_service::process_join_request;
use sea_orm::DbConn;
use validator::Validate;

pub async fn join_handler(
    Extension(db_conn): Extension<DbConn>,
    Json(payload): Json<JoinRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation failed: {e}")));
    }

    process_join_request(&payload.email, &db_conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {e}")))?;

    Ok(StatusCode::OK)
}