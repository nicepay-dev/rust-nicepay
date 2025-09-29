use crate::models::snap::check_status_direct_debit_model::CheckStatusDirectDebitModel;
use crate::models::snap::refund_direct_debit_model::RefundDirectDebitModel;
use crate::models::snap::regist_direct_debit_model::RegistDirectDebitModel;
use crate::utils::config::Config;
use crate::utils::helper::Helper;
use reqwest::{Client, Error, Response};
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

pub struct DirectDebitSNAPRequester<'a> {
    pub config: &'a Config,
}

impl<'a> DirectDebitSNAPRequester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn request_regist_direct_debit(
        &self,
        req_debit: RegistDirectDebitModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/debit/payment-host-to-host", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_debit).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/debit/payment-host-to-host:{}:{}:{}",
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
                .json(&req_debit)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_check_status_direct_debit(
        &self,
        req_debit: CheckStatusDirectDebitModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/debit/status", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_debit).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/debit/status:{}:{}:{}",
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
                .json(&req_debit)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_refund_direct_debit(
        &self,
        req_debit: RefundDirectDebitModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/debit/refund", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_debit).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/debit/refund:{}:{}:{}",
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
                .json(&req_debit)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }
}
