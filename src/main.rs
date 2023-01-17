#![forbid(unsafe_code)]

use axum::Router;
use reqwest::StatusCode;
use std::net::SocketAddr;
use axum::routing::get;
use axum::response::IntoResponse;
use axum::response::Html;
use axum::response::Redirect;
use serde::Deserialize;
use axum::extract::Query;

#[derive(Deserialize)]
struct CallbackValue {
    code: Option<String>,
    state: String,
    error: Option<String>,
    error_description: Option<String>
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent(
            "https://notify-bot.line.me/oauth/authorize?response_type=code&client_id=tQJrXoXNwVParKfUQ0LZzA&redirect_uri=https://share.eztw.in/api/callback&scope=notify&state=12345"
        ) }))
        .route("/api/callback", get(callback));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server startup failed.");
}

async fn callback(param: Query<CallbackValue>) -> impl IntoResponse {
    match &param.0.code {
        Some(code) => (StatusCode::OK, Html::from(format!("State: {}. Token: {}",param.0.state, code))),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Html::from(format!("Error occured: {}, description from LINE: {}", param.0.error.unwrap_or("No error code".to_string()), param.0.error_description.unwrap_or("No error description.".to_string())))),
    }
}

