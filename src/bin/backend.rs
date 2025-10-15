use axum::{
    routing::get, Json, Router
};

use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;
use http::Method;

use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/data", get(get_data))
        .layer(ServiceBuilder::new().layer(cors_layer));

    let port = "3000";
    let ip_address_string = format!("127.0.0.1:{port}");
    let ip_address = ip_address_string.as_str();
    let listener = tokio::net::TcpListener::bind(ip_address).await.unwrap();
    println!("Backend server running at http://{}.", ip_address);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello world!"
}

async fn get_data() -> Json<Value> {
    Json(json!({"data": 42}))
}