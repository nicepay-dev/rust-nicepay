use crate::models::snap::cancel_vacct_model::CancelVacctModel;
use crate::models::snap::create_vacct_model::CreateVacctModel;
use crate::models::snap::inquiry_vacct_model::InquiryVacctModel;
use crate::utils::config::Config;
use crate::utils::helper::Helper;
use reqwest::{Client, Error, Response};
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

pub struct VacctSNAPRequester<'a> {
    pub config: &'a Config,
}

impl<'a> VacctSNAPRequester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn request_create_va(
        &self,
        req_vacct: CreateVacctModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/transfer-va/create-va", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_vacct).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/transfer-va/create-va:{}:{}:{}",
            access_token, encoded_payload, x_timestamp
        );
        let x_signature = Helper::hash_with_hmac256(&self.config.client_secret, &string_to_sign)
            .expect("Failed to sign");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token).parse().unwrap(),
        );
        headers.insert("X-TIMESTAMP", x_timestamp.parse().unwrap());
        headers.insert("X-PARTNER-ID", self.config.client_id.parse().unwrap());
        headers.insert("X-SIGNATURE", x_signature.parse().unwrap());
        headers.insert("X-EXTERNAL-ID", x_external_id.parse().unwrap());
        headers.insert("CHANNEL-ID", self.config.channel_id.parse().unwrap());

        let client = Client::new();

        let fut = async move {
            let response = client
                .post(url)
                .headers(headers)
                .json(&req_vacct)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_inquiry_va(
        &self,
        req_vacct: InquiryVacctModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/transfer-va/status", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_vacct).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/transfer-va/status:{}:{}:{}",
            access_token, encoded_payload, x_timestamp
        );
        let x_signature = Helper::hash_with_hmac256(&self.config.client_secret, &string_to_sign)
            .expect("Failed to sign");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token).parse().unwrap(),
        );
        headers.insert("X-TIMESTAMP", x_timestamp.parse().unwrap());
        headers.insert("X-PARTNER-ID", self.config.client_id.parse().unwrap());
        headers.insert("X-SIGNATURE", x_signature.parse().unwrap());
        headers.insert("X-EXTERNAL-ID", x_external_id.parse().unwrap());
        headers.insert("CHANNEL-ID", self.config.channel_id.parse().unwrap());

        let client = Client::new();

        let fut = async move {
            let response = client
                .post(url)
                .headers(headers)
                .json(&req_vacct)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_cancel_va(
        &self,
        req_vacct: CancelVacctModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/transfer-va/delete-va", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_vacct).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "DELETE:/api/v1.0/transfer-va/delete-va:{}:{}:{}",
            access_token, encoded_payload, x_timestamp
        );
        let x_signature = Helper::hash_with_hmac256(&self.config.client_secret, &string_to_sign)
            .expect("Failed to sign");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token).parse().unwrap(),
        );
        headers.insert("X-TIMESTAMP", x_timestamp.parse().unwrap());
        headers.insert("X-PARTNER-ID", self.config.client_id.parse().unwrap());
        headers.insert("X-SIGNATURE", x_signature.parse().unwrap());
        headers.insert("X-EXTERNAL-ID", x_external_id.parse().unwrap());
        headers.insert("CHANNEL-ID", self.config.channel_id.parse().unwrap());

        let client = Client::new();

        let fut = async move {
            let response = client
                .post(url)
                .headers(headers)
                .json(&req_vacct)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }
}
