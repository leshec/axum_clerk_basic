use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    response::Html,
    routing::{get, post},
    Router,
};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};
use shuttle_runtime::SecretStore;
use std::sync::Arc;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let clerk_secret = secrets
        .get("CLERK_SECRET_KEY")
        .expect("CLERK_SECRET_KEY not set");

    let config = ClerkConfiguration::new(None, None, Some(clerk_secret.to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/index", get(index))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk),
            None,
            true,
        ))
        .route_service("/", ServeDir::new("static"));

    Ok(app.into())
}
// .route("/api/protected", post(protected_handler))

async fn index() -> &'static str {
    "hello richy poo bear"
}
