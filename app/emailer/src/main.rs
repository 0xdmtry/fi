use axum::ServiceExt;
use emailer::app;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    println!("🚀 Emailer running on http://localhost:8001");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001")
        .await
        .expect("Failed to bind port");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed");
}
