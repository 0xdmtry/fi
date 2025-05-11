use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateAtaRequest {
    pub user_id: Uuid,

    #[validate(length(min = 1))]
    pub token_mint: String,

    pub wallet_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CreateAtaResponse {
    pub ata_address: String,
    pub tx_signature: String,
}