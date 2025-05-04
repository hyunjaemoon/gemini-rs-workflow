use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use std::env;
use std::sync::Arc;
use std::fs;
use tower_http::cors::{Any, CorsLayer};
use webbrowser;

use crate::gemini::{ChatRequest, ChatResponse, GeminiClient};

pub struct AppState {
    gemini_client: GeminiClient,
}

pub async fn run_server() {
    // Load environment variables
    dotenv::dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create shared state
    let state = Arc::new(AppState {
        gemini_client: GeminiClient::new(api_key),
    });

    // Build our application with routes
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/chat", post(handle_chat))
        .layer(cors)
        .with_state(state);

    // Run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    
    // Open the browser
    if let Err(e) = webbrowser::open("http://127.0.0.1:3000") {
        eprintln!("Failed to open browser: {}", e);
    }
    
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> Html<String> {
    let html = fs::read_to_string("index.html").unwrap_or_else(|_| {
        String::from("Error: index.html not found")
    });
    Html(html)
}

async fn handle_chat(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Extract the text from the request
    let text = payload
        .contents
        .first()
        .and_then(|content| content.parts.first())
        .map(|part| part.text.as_str())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid request format".to_string(),
            )
        })?;

    // Generate content using Gemini client
    let response_text = state.gemini_client.generate_content(text)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(ChatResponse {
        response: response_text,
    }))
} 