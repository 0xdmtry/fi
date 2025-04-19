use anyhow::Result;
use super::emailer_service::Emailer;

pub struct MailhogEmailer;

impl MailhogEmailer {
    pub fn new() -> Self {
        Self
    }
}

impl Emailer for MailhogEmailer {
    fn send_passcode(&self, email: &str, passcode: &str) -> Result<()> {
        println!("[MAILHOG] Sending passcode {} to {}", passcode, email);
        Ok(())
    }
}