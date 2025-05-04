use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub contents: Vec<Content>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
}

pub struct GeminiClient {
    api_key: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn generate_content(&self, text: &str) -> Result<String, String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            self.api_key
        );

        let request_body = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": text
                }]
            }]
        });

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let response_text = response
            .text()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response_text)
    }
} 