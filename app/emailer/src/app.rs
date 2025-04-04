use axum::{Router, Extension};
use crate::routes;
use crate::services::{Emailer, MailhogEmailer};
use std::sync::Arc;
use std::env;

pub fn create_app() -> Router {

    let emailer: Arc<dyn Emailer> = match env::var("EMAIL_PROVIDER").as_deref() {
        _ => Arc::new(MailhogEmailer::new()),
    };


    Router::new()
        .nest("/v1", routes::v1::routes()) // version prefix
        .layer(Extension(emailer))
}
