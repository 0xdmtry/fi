use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct JoinRequest {
    #[validate(email)]
    #[validate(length(min=5, max=254))]
    pub email: String,
}