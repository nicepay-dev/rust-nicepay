# NicePay Rust SDK - Complete Integration Guide

A comprehensive Rust library for integrating with NicePay's payment platform, supporting virtual accounts, direct debit, QR payments, payouts, and more through the SNAP API.

## Overview

This Rust SDK provides a complete interface for integrating with NicePay's payment platform, supporting multiple payment methods including virtual accounts, direct debit, QR payments, and payouts through a unified, type-safe API.

## Features

- ✅ **Virtual Account Management** - Create, inquire, and delete virtual accounts
- ✅ **Direct Debit** - Process direct debit transactions
- ✅ **QR Payments** - Generate and manage QR code payments
- ✅ **Payout Services** - Process payouts and transfers
- ✅ **Async/Await Support** - Non-blocking operations with async/await
- ✅ **Automatic Security** - Built-in signature generation and validation
- ✅ **Flexible Configuration** - Support for both development and production environments

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-nicepay = { git = "https://github.com/nicepay-dev/rust-nicepay.git" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Basic Setup

```rust
use rust_nicepay::{config::Config, accessToken::AccessToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize configuration
    let config = Config {
        client_id: "your_client_id".to_string(),
        client_secret: "your_client_secret".to_string(),
        private_key: "your_private_key".to_string(),
        merchant_key: "your_merchant_key".to_string(),
        channel_id: "your_channel_id".to_string(),
        is_production: false, // Set to true for production
        is_cloud_server: false, // Set to true for cloud server
        ..Default::default()
    };

    // Get access token
    let access_token = AccessToken::get_access_token(&config).await?;

    Ok(())
}
```

## Configuration

### Environment Variables

```bash
export NICEPAY_CLIENT_ID=your_client_id
export NICEPAY_CLIENT_SECRET=your_client_secret
export NICEPAY_PRIVATE_KEY=your_private_key
export NICEPAY_MERCHANT_KEY=your_merchant_key
export NICEPAY_CHANNEL_ID=your_channel_id
export NICEPAY_IS_PRODUCTION=false
export NICEPAY_IS_CLOUD_SERVER=false
```

## Payment Methods

### 1. Virtual Account Operations

#### Creating a Virtual Account

```rust
use rust_nicepay::{
    request_vacct::RequestVacct,
    model::create_vacct_model::{CreateVacctBuilder, AdditionalInfo, TotalAmount},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let config = Config {
        client_id: "your_client_id".to_string(),
        client_secret: "your_client_secret".to_string(),
        private_key: "your_private_key".to_string(),
        merchant_key: "your_merchant_key".to_string(),
        channel_id: "your_channel_id".to_string(),
        is_production: false, // Set to true for production
        is_cloud_server: false, // Set to true for cloud server
        ..Default::default()
    };

    let vacct_client = RequestVacct::new(&config);
    let access_token = AccessToken::get_access_token(&config).await?;

    let builder = CreateVacctBuilder::new()
        .partner_service_id("VA".to_string())
        .customer_no("CUST123456".to_string())
        .virtual_account_no("1234567890123456".to_string())
        .virtual_account_name("John Doe".to_string())
        .trx_id("TXN-2024-001".to_string())
        .total_amount(TotalAmount {
            value: "100000.00".to_string(),
            currency: "IDR".to_string(),
        })
        .additional_info(vec![AdditionalInfo {
            bank_cd: "BNI".to_string(),
            goods_nm: "Payment for Order #12345".to_string(),
            db_process_url: "https://your-callback.com/webhook".to_string(),
            vacct_valid_dt: "20241231".to_string(),
            vacct_valid_tm: "235959".to_string(),
            ms_id: "MERCHANT123".to_string(),
            ms_fee: "1000.00".to_string(),
            ms_fee_type: "flat".to_string(),
            mb_fee: "500.00".to_string(),
            mb_fee_type: "flat".to_string(),
        }]);

    let response = vacct_client.request_generate_vacct(builder, "EXT-001", &access_token).await?;

    println!("VA Created: {}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
```

#### Inquiring Virtual Account Details

```rust
let inquiry_builder = InquiryVacctBuilder::new()
    .partner_service_id("VA".to_string())
    .customer_no("CUST123456".to_string())
    .virtual_account_no("1234567890123456".to_string())
    .inquiry_request_id("INQ-001".to_string())
    .additional_info(vec![AdditionalInfo {
        trx_id: "TXN-2024-001".to_string(),
        t_xid_va: "VA-2024-001".to_string(),
        total_amount: TotalAmount {
            value: "100000.00".to_string(),
            currency: "IDR".to_string(),
        },
    }]);

let response = vacct_client.request_inquiry_vacct(inquiry_builder, "EXT-INQ-001", &access_token).await?;
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support, please contact:

- Email: it@nicepay.co.id
- Documentation: https://docs.nicepay.co.id
- Issues: https://github.com/nicepay-dev/rust-nicepay/issues
