use axum::{response::Html, routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};
use shuttle_runtime::SecretStore;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let clerk_secret = secrets
        .get("CLERK_SECRET_KEY")
        .expect("CLERK_SECRET_KEY not set");

    let config = ClerkConfiguration::new(None, None, Some(clerk_secret.to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        //protected routes
        .route("/amloggedin", get(amloggedin))
        .route("/protected_route", get(protected_route))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk),
            None,
            true,
        ))
        //unprotected routes
        .route_service("/", ServeDir::new("static"))
        .route("/unprotected", get(unprotected));

    Ok(app.into())
}

async fn unprotected() -> Html<String> {
    Html("this info is unprotected".to_string())
}

async fn amloggedin() -> &'static str {
    "protected route: hello you are here you are logged in"
}

async fn protected_route() -> Html<String> {
    Html("hello from info as a protected route ".to_string())
}
