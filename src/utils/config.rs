// use serde::{Debug, Deserialize, Serialize};
use std::collections::HashMap;

pub struct Config {
    pub is_production: bool,
    pub private_key: String,
    pub client_secret: String,
    pub client_id: String,
    pub is_cloud_server: bool,
    pub merchant_key: String,
    pub channel_id: String,
}

impl Config {
    pub const SNAP_DEV_BASE_URL: &'static str = "https://dev.nicepay.co.id/nicepay";
    pub const SNAP_PROD_BASE_URL: &'static str = "https://www.nicepay.co.id/nicepay";
    pub const SNAP_DEV_CLOUD_BASE_URL: &'static str = "https://dev-services.nicepay.co.id/nicepay";
    pub const SNAP_PROD_CLOUD_BASE_URL: &'static str = "https://services.nicepay.co.id/nicepay";

    pub fn new() -> Self {
        Self {
            is_production: false,
            private_key: String::new(),
            client_secret: String::new(),
            client_id: String::new(),
            is_cloud_server: false,
            merchant_key: String::new(),
            channel_id: String::new(),
        }
    }

    pub fn with_options(options: &HashMap<&str, String>) -> Self {
        let mut config = Self::new();
        config.set_configuration(options);
        config
    }

    pub fn get_configuration(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("is_production".to_string(), self.is_production.to_string());
        config.insert("private_key".to_string(), self.private_key.clone());
        config.insert("client_secret".to_string(), self.client_secret.clone());
        config.insert("client_id".to_string(), self.client_id.clone());
        config.insert(
            "is_cloud_server".to_string(),
            self.is_cloud_server.to_string(),
        );
        config.insert("merchant_key".to_string(), self.merchant_key.clone());
        config.insert("channel_id".to_string(), self.channel_id.clone());
        config
    }

    pub fn set_configuration(&mut self, options: &HashMap<&str, String>) {
        if let Some(is_production) = options.get("is_production") {
            self.is_production = is_production.parse().unwrap_or(false);
        }
        if let Some(private_key) = options.get("private_key") {
            self.private_key = private_key.clone();
        }
        if let Some(client_secret) = options.get("client_secret") {
            self.client_secret = client_secret.clone();
        }
        if let Some(client_id) = options.get("client_id") {
            self.client_id = client_id.clone();
        }
        if let Some(is_cloud_server) = options.get("is_cloud_server") {
            self.is_cloud_server = is_cloud_server.parse().unwrap_or(false);
        }
        if let Some(merchant_key) = options.get("merchant_key") {
            self.merchant_key = merchant_key.clone();
        }
        if let Some(channel_id) = options.get("channel_id") {
            self.channel_id = channel_id.clone();
        }
    }

    pub fn get_snap_api_base_url(&self) -> &'static str {
        if self.is_cloud_server {
            if self.is_production {
                Self::SNAP_PROD_CLOUD_BASE_URL
            } else {
                Self::SNAP_DEV_CLOUD_BASE_URL
            }
        } else {
            if self.is_production {
                Self::SNAP_PROD_BASE_URL
            } else {
                Self::SNAP_DEV_BASE_URL
            }
        }
    }
}
