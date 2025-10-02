use crate::utils::config::Config;
use crate::utils::helper::Helper;
use reqwest::{Client, Error, Response};
use serde_json::json;
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

pub struct AccessTokenRequester<'a> {
    pub config: &'a Config,
}

impl<'a> AccessTokenRequester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn request_access_token(&self) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/v1.0/access-token/b2b", base_url);

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!("{}|{}", self.config.client_id, x_timestamp);
        let x_signature =
            Helper::sign_with_private_key_string(&self.config.private_key, &string_to_sign)
                .expect("Failed to sign");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert("X-TIMESTAMP", x_timestamp.parse().unwrap());
        headers.insert("X-CLIENT-KEY", self.config.client_id.parse().unwrap());
        headers.insert("X-SIGNATURE", x_signature.parse().unwrap());

        let body = json!({
            "grantType": "client_credentials",
            "additionalInfo": json!({})
        });

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).headers(headers).json(&body).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }
}
