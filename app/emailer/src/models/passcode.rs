use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SendPasscodeRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 4, max = 16))]
    pub passcode: String,
}
