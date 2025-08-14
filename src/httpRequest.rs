use reqwest::{Client, Method, Response, Error as ReqwestError};
use serde_json::Value;
use thiserror::Error;
use std::time::Duration;

#[derive(Error, Debug)]
pub enum NicepayError {
    #[error("Failed to parse body as JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    
    #[error("HTTP request error: {0}")]
    RequestError(#[from] ReqwestError),
    
    #[error("API error response: status {status}, body: {body}")]
    ApiErrorResponse {
        status: reqwest::StatusCode,
        body: String,
    },
    
    #[error("Timeout error")]
    TimeoutError,
}

pub struct HttpRequest {
    client: Client,
}

impl HttpRequest {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn request(
        &self,
        headers: Vec<(&str, &str)>,
        request_url: &str,
        request_body: Value,
        http_method: Method,
    ) -> Result<Value, NicepayError> {
        // Convert the JSON Value to a string for the request
        let body_string = serde_json::to_string(&request_body)?;

        // Build the request
        let mut request = self.client
            .request(http_method, request_url)
            .headers({
                let mut header_map = reqwest::header::HeaderMap::new();
                for (key, value) in headers {
                    header_map.insert(
                        reqwest::header::HeaderName::from_bytes(key.as_bytes())?,
                        reqwest::header::HeaderValue::from_str(value)?,
                    );
                }
                header_map
            })
            .body(body_string)
            .timeout(Duration::from_secs(30)); // Set a timeout

        // Execute the request with retry logic for timeouts
        let response = match request.try_clone().unwrap().send().await {
            Ok(res) => res,
            Err(err) => {
                if err.is_timeout() {
                    // Retry once on timeout
                    request.send().await?
                } else {
                    return Err(NicepayError::from(err));
                }
            }
        };

        // Handle the response
        if response.status().is_success() {
            let response_body: Value = response.json().await?;
            Ok(response_body)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(NicepayError::ApiErrorResponse { status, body })
        }
    }
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self::new()
    }
}