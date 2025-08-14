use crate::accessToken::AccessToken;
use crate::config::Config;
use crate::helper::Helper;
use crate::httpRequest::{HttpRequest, NicepayError};
use crate::model::createVacctModel::{CreateVacctBuilder, VacctRequest};
use crate::model::deleteVacctModel::DeleteVacctBuilder;
use crate::model::inquiryVacctModel::{InquiryVacctBuilder, VacctRequest};
use reqwest::Method;
use serde_json::{json, Value};

/// Virtual Account client for managing virtual account operations
pub struct RequestVacct {
    config: Config,
}

impl RequestVacct {
    pub fn new(config: &Config) -> Self {
        Self { config }
    }

    pub async fn request_generate_vacct(
        &self,
        builder: CreateVacctBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/transfer-va/status";
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

    pub async fn request_inquiry_vacct(
        &self,
        builder: InquiryVacctBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/transfer-va/status";
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

    pub async fn request_delete_vacct(
        &self,
        builder: DeleteVacctBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/transfer-va/delete-va";
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
