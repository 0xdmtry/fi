use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SignTransactionRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1))]
    pub transaction_base64: String,
    pub wallet_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SignTransactionResponse {
    pub signed_transaction_base64: String,
    pub signature: String,
    pub wallet_address: Option<String>,
    pub wallet_public_key: Option<String>,
}
