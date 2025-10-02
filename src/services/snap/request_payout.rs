use crate::models::snap::approve_payout_model::ApprovePayoutModel;
use crate::models::snap::cancel_payout_model::CancelPayoutModel;
use crate::models::snap::check_balance_payout_model::CheckBalancePayoutModel;
use crate::models::snap::inquiry_payout_model::InquiryPayoutModel;
use crate::models::snap::regist_payout_model::RegistPayoutModel;
use crate::models::snap::reject_payout_model::RejectPayoutModel;
use crate::utils::config::Config;
use crate::utils::helper::Helper;
use reqwest::{Client, Error, Response};
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

pub struct PayoutSNAPRequester<'a> {
    pub config: &'a Config,
}

impl<'a> PayoutSNAPRequester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn request_regist_payout(
        &self,
        req_payout: RegistPayoutModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/transfer/registration", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_payout).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/transfer/registration:{}:{}:{}",
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
                .json(&req_payout)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_approve_payout(
        &self,
        req_payout: ApprovePayoutModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/transfer/approve", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_payout).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/transfer/approve:{}:{}:{}",
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
                .json(&req_payout)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_inquiry_payout(
        &self,
        req_payout: InquiryPayoutModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/transfer/inquiry", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_payout).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/transfer/inquiry:{}:{}:{}",
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
                .json(&req_payout)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_reject_payout(
        &self,
        req_payout: RejectPayoutModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/transfer/reject", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_payout).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/transfer/reject:{}:{}:{}",
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
                .json(&req_payout)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_cancel_payout(
        &self,
        req_payout: CancelPayoutModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/transfer/cancel", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_payout).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/transfer/cancel:{}:{}:{}",
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
                .json(&req_payout)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_check_balance_payout(
        &self,
        req_payout: CheckBalancePayoutModel,
        access_token: &str,
        x_external_id: &str,
    ) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/v1.0/balance-inquiry", base_url);
        let encoded_payload =
            Helper::hex_encoded_payload(&req_payout).expect("Failed to hex encode");

        let x_timestamp = Helper::get_formatted_date();
        let string_to_sign = format!(
            "POST:/api/v1.0/balance-inquiry:{}:{}:{}",
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
                .json(&req_payout)
                .send()
                .await?;
            Ok(response)
        };

        Box::pin(fut)
    }
}
