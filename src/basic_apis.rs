use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

// Shared struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

// GET handler
async fn hello() -> Json<Message> {
    Json(Message {
        message: "Welcome to Rust API! Hi Swarna Lakshmi Bese!!!!!!".to_string(),
    })
}

// POST handler
async fn echo(Json(payload): Json<Message>) -> Json<Message> {
    Json(Message {
        message: format!("You said: {}", payload.message),
    })
}

// Exported function to create routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/echo", post(echo))
}
