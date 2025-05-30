use walletor::app;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    println!("🚀 Api running on http://localhost:8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind port");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed");
}
