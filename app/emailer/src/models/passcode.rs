use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SendPasscodeRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 4, max = 8))]
    pub passcode: String,
}
