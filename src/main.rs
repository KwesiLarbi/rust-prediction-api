use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build app with single route
    println!("Building app...");
    let app = Router::new().route("/health-check", get(health));

    // run app
    println!("running app...");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "\nHello, World!\n\n"
}
