use crate::config::Config;
use crate::helper::Helper;
use crate::httpRequest::{HttpRequest, NicepayError};
use reqwest::Method;
use serde_json::{json, Value};

pub struct AccessTokenRequester<'a> {
    pub config: &'a Config,
}

impl<'a> AccessTokenRequester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn request_access_token(&self) -> Result<Value, NicepayError> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/v1.0/access-token/b2b", base_url);

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!("{}|{}", self.config.client_id, x_timestamp);
        let x_signature =
            Helper::get_signature_access_token(&self.config.private_key, &string_to_sign).map_err(
                |e| {
                    NicepayError::RequestError(reqwest::Error::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    )))
                },
            )?;

        let headers = vec![
            ("Content-Type", "application/json"),
            ("X-TIMESTAMP", &x_timestamp),
            ("X-CLIENT-KEY", &self.config.client_id),
            ("X-SIGNATURE", &x_signature),
        ];

        let body = json!({
            "grantType": "client_credentials",
            "additionalInfo": json!({})
        });

        let http = HttpRequest::new();
        http.request(headers, &url, body, Method::POST).await
    }
}
