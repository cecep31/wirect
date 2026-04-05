use axum::{
    Router,
    extract::State,
    http::Uri,
    response::{IntoResponse, Redirect, Response},
    routing::{any, get},
};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    new_domain: Arc<String>,
}

async fn domain_redirect_handler(
    State(state): State<AppState>,
    uri: Uri,
) -> Response {
    let path = uri.path();
    let query = uri.query().unwrap_or("");
    let base = state.new_domain.trim_end_matches('/');

    let new_url = if query.is_empty() {
        format!("{base}{path}")
    } else {
        format!("{base}{path}?{query}")
    };

    Redirect::permanent(&new_url).into_response()
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let new_domain = std::env::var("NEW_DOMAIN")
        .expect("NEW_DOMAIN must be set in .env or environment");
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("127.0.0.1:{port}");

    let state = AppState {
        new_domain: Arc::new(new_domain),
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", any(domain_redirect_handler))
        .route("/{*path}", any(domain_redirect_handler))
        .with_state(state.clone());

    println!("Starting redirect server on http://{bind_addr}");
    println!("Redirecting all incoming requests to {}", state.new_domain);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind TCP listener");
    axum::serve(listener, app).await.unwrap();
}
