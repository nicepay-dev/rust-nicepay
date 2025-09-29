# rust-nicepay

A Rust library for integrating with Nicepay payment services, providing easy access to SNAP and V2 APIs for direct payments, payouts, redirects, and more.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-nicepay = "0.1.0"
```

## Usage

First, create a configuration with your Nicepay credentials:

```rust
use rust_nicepay::utils::config::Config;
use std::collections::HashMap;

let mut options = HashMap::new();
options.insert("client_id", "YOUR_CLIENT_ID".to_string());
options.insert("private_key", "YOUR_PRIVATE_KEY".to_string());
options.insert("client_secret", "YOUR_CLIENT_SECRET".to_string());
options.insert("is_production", "false".to_string()); // Set to "true" for production
options.insert("is_cloud_server", "false".to_string());
options.insert("channel_id", "YOUR_CHANNEL_ID".to_string());

let config = Config::with_options(&options);
```

### Request Access Token

To request an access token for SNAP API:

```rust
use rust_nicepay::services::snap::request_access_token::AccessTokenRequester;

let requester = AccessTokenRequester::new(&config);
let response = requester.request_access_token().await?;
```

### Create Virtual Account

To create a virtual account:

```rust
use rust_nicepay::models::snap::create_vacct_model::{AdditionalInfo, CreateVacctModel, TotalAmount};
use rust_nicepay::services::snap::request_vacct::VacctSNAPRequester;

let vacct_requester = VacctSNAPRequester::new(&config);

let req_vacct = CreateVacctModel {
    partner_service_id: Some("".to_string()),
    customer_no: Some("".to_string()),
    virtual_account_no: Some("".to_string()),
    virtual_account_name: Some("Testing".to_string()),
    trx_id: Some("OrderNo-1234".to_string()),
    total_amount: Some(TotalAmount {
        value: "10000.00".to_string(),
        currency: "IDR".to_string(),
    }),
    additional_info: Some(AdditionalInfo {
        bank_cd: "BMRI".to_string(),
        goods_nm: "Good Name".to_string(),
        db_process_url: "https://merchant.domain.com".to_string(),
        vacct_valid_dt: "".to_string(),
        vacct_valid_tm: "".to_string(),
        ms_id: "".to_string(),
        ms_fee: "".to_string(),
        ms_fee_type: "".to_string(),
        mb_fee: "".to_string(),
        mb_fee_type: "".to_string(),
    }),
};

let access_token = "YOUR_ACCESS_TOKEN";
let x_external_id = "unique_external_id";

let response = vacct_requester.request_create_va(req_vacct, access_token, x_external_id).await?;
```

## Supported APIs

- SNAP API: Access tokens, virtual accounts, QR codes, payouts, direct debit
- V2 API: Direct payments, redirects, payouts

Refer to the `services` module for all available requesters and models.
