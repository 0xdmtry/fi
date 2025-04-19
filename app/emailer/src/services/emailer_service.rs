use anyhow::Result;

pub trait Emailer: Send + Sync {
    fn send_passcode(&self, email: &str, passcode: &str) -> Result<()>;
}