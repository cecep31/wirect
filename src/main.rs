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
    base_url: Arc<str>,
}

impl AppState {
    fn redirect_location(&self, uri: &Uri) -> String {
        let base = &*self.base_url;
        let path = uri.path();
        let query = uri.query().unwrap_or("");

        if query.is_empty() {
            format!("{base}{path}")
        } else {
            format!("{base}{path}?{query}")
        }
    }
}

async fn redirect_handler(State(state): State<AppState>, uri: Uri) -> Response {
    let location = state.redirect_location(&uri);
    Redirect::permanent(&location).into_response()
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let new_domain = std::env::var("NEW_DOMAIN")
        .expect("NEW_DOMAIN must be set in .env or environment");
    let base_url: Arc<str> = Arc::from(new_domain.trim_end_matches('/').to_string());

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let bind_addr = format!("{host}:{port}");

    let state = AppState { base_url };

    println!("Starting redirect server on http://{bind_addr}");
    println!("Redirecting all incoming requests to {}", state.base_url);

    let app = Router::new()
        .route("/health", get(health_check))
        .fallback(any(redirect_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind TCP listener");
    axum::serve(listener, app).await.unwrap();
}
