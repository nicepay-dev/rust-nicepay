use crate::config::Config;
use crate::helper::Helper;
use crate::httpRequest::{HttpRequest, NicepayError};
use reqwest::Method;
use serde_json::{json, Value};

pub struct RequestQR {
    config: Config,
}

impl RequestQR {
    pub fn new(config: &Config) -> Self {
        Self { config }
    }

    pub async fn request_generate_qris(
        &self,
        builder: GenerateQRBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/qr/qr-mpm-generate";
        let string_to_sign = format!(
            "{}:{}:{}:{}:{}",
            "POST",
            end_point,
            access_token,
            serde_json::to_string(&body).unwrap(),
            timestamp
        );
        let x_signature = Helper::generate_signature(&string_to_sign, &self.config.client_secret);

        let headers = vec![
            ("Content-Type", "application/json"),
            ("X-TIMESTAMP", &timestamp),
            ("X-CLIENT-KEY", &self.config.client_id),
            ("X-SIGNATURE", &x_signature),
            ("X-ACCESS-TOKEN", &access_token),
            ("CHANNEL-ID", &self.config.channel_id),
            ("X-EXTERNAL-ID", external_id),
        ];
        let http = HttpRequest::new();
        http.request(headers, &url, body, Method::POST).await
    }

    pub async fn request_inquiry_qris(
        &self,
        builder: QueryQRBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/qr/qr-mpm-query";
        let string_to_sign = format!(
            "{}:{}:{}:{}:{}",
            "POST",
            end_point,
            access_token,
            serde_json::to_string(&body).unwrap(),
            timestamp
        );
        let x_signature = Helper::generate_signature(&string_to_sign, &self.config.client_secret);

        let headers = vec![
            ("Content-Type", "application/json"),
            ("X-TIMESTAMP", &timestamp),
            ("X-CLIENT-KEY", &self.config.client_id),
            ("X-SIGNATURE", &x_signature),
            ("X-ACCESS-TOKEN", &access_token),
            ("CHANNEL-ID", &self.config.channel_id),
            ("X-EXTERNAL-ID", external_id),
        ];
        let http = HttpRequest::new();
        http.request(headers, &url, Value::Null, Method::POST).await
    }

    pub async fn request_refund_qris(
        &self,
        builder: RefundQRBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/qr/qr-mpm-refund";
        let string_to_sign = format!(
            "{}:{}:{}:{}:{}",
            "POST",
            end_point,
            access_token,
            serde_json::to_string(&body).unwrap(),
            timestamp
        );
        let x_signature = Helper::generate_signature(&string_to_sign, &self.config.client_secret);

        let headers = vec![
            ("Content-Type", "application/json"),
            ("X-TIMESTAMP", &timestamp),
            ("X-CLIENT-KEY", &self.config.client_id),
            ("X-SIGNATURE", &x_signature),
            ("X-ACCESS-TOKEN", &access_token),
            ("CHANNEL-ID", &self.config.channel_id),
            ("X-EXTERNAL-ID", external_id),
        ];
        let http = HttpRequest::new();
        http.request(headers, &url, Value::Null, Method::DELETE)
            .await
    }
}
