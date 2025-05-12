use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct FundWsolRequest {
    pub user_id: Uuid,
    pub amount_sol: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FundWsolResponse {
    pub ata_address: String,
    pub tx_signature: String,
}
