use crate::config::Config;
use crate::helper::Helper;
use crate::httpRequest::{HttpRequest, NicepayError};
use crate::model::approvePayoutModel::{ApprovePayoutModel, ApprovePayoutModelBuilder};
use crate::model::cancelPayoutModel::{CancelPayoutModel, CancelPayoutModelBuilder};
use crate::model::checkBalanceModel::{CheckBalanceModel, CheckBalanceModelBuilder};
use crate::model::inquiryPayoutModel::{InquiryPayoutModel, InquiryPayoutModelBuilder};
use crate::model::registerPayoutModel::{RegisterPayoutModel, RegisterPayoutModelBuilder};
use crate::model::rejectPayoutModel::{RejectPayoutModel, RejectPayoutModelBuilder};
use reqwest::Method;
use serde_json::{json, Value};

pub struct RequestPayout {
    config: Config,
}

impl RequestPayout {
    pub fn new(config: &Config) -> Self {
        Self { config }
    }

    pub async fn request_regist_payout(
        &self,
        builder: RegisterPayoutModelBuilder,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/transfer/registration";
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

    pub async fn request_approve_payout(
        &self,
        builder: ApprovePayoutModelBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/transfer/approve";
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

    pub async fn request_inquiry_payout(
        &self,
        builder: InquiryPayoutModelBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/transfer/inquiry";
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

    pub async fn request_cancel_payout(
        &self,
        builder: CancelPayoutModelBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/transfer/cancel";
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

    pub async fn request_reject_payout(
        &self,
        builder: RejectPayoutModelBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/transfer/reject";
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

    pub async fn request_balance_inquiry_payout(
        &self,
        builder: CheckBalanceModelBuilder,
        external_id: &str,
        access_token: &str,
    ) -> Result<Value, NicepayError> {
        let url = self.config.get_snap_api_base_url();
        let timestamp = Helper::get_timestamp();

        let body = serde_json::to_value(&builder)?;
        let end_point = "/api/v1.0/balance-inquiry";
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
}
