mod app;
mod routes;
mod services;
mod handlers;
mod config;
mod error;
mod models;
mod db;

#[tokio::main]
async fn main() {
    let app = app::create_app();

    println!("🚀 Emailer running on http://localhost:8001");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001")
        .await
        .expect("Failed to bind port");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}