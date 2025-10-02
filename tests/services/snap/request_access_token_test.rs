pub mod models;
pub mod services;
pub mod utils;

#[tokio::test]
async fn test_request_access_token_success() {
    let config = Config {
        client_id: "IONPAYTEST".to_string(),
        private_key: "MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBAKKFhxPzra4tbtTfEQhlPrYJ5VgfMT23T9DyD7LlLzxvOp0zRsAbgLbmroIwLpAjw6WI0naaBc75Y4xEb50UrrVyC8k/PZfMhOrLZ7LRxBM+KD3J1UFs+mG1yDdMlM9yGHSUC3ttBmqC6plB1qNR5tMUjWn9vqVP5ck0LfUDG/OVAgMBAAECgYAAuRO9FVDiic1Vi+JZUBQeVkEQ21qqQUizaS4doYs7P6h8W7v3+EU3k5q5ZJngbM2i/9/QVciIbm+03MN5Akck8LwwtgeaCjlwGVaianeysoS6/6O7kujNvjTtqL3TZccx03C3LR2irXeLR9FKLApTK3t6N6k23zwX153Kp6DCIQJBANgUqSWa5Jt5jkhJoDUSLR4t8NfOLy7ncBEYY/Ey5WgAosgqTZaYnnK/zT2iL9Qm7nefRcYdV8+aMt/MXsIzlckCQQDAi9ZwHmUEMTAKss6dahmRZ+dYLvwEuTaJdgvYpPBMxzR3R9h1EJsIa7TeLedYyJtQMzDbk17JRJydSylolkVtAkEAtsHSxnGZhT5sbwuGqxxyeKIWoBBNq/gnuu3MKgcILMzM4UuWUBdJfHhpGQYCOgerjhVyKDxNNtOOz+bFBrAmsQJAeJ/iYnEYNc0e3MTyHbnXdLmUPDGLHuZtXSZ/+2QxSthNbSCsYYJarabUM5Csa3mZm1/Gjvi/G/YI652nvmbN7QJAUWusTqf3g3/X6Mwpoxz3ekZh0U59yTQIjgha4L4baOlBm9KulwZFcMUiPHfwyEVk46/TPe/7O0JJLgE93eeOUg==".to_string(),
        is_cloud_server: false,
        is_production: false,
        client_secret: "1427e89301715e7c856996bc59e27970be322877004b3fe3340b48969e2288d6".to_string(),
        channel_id: "IONPAYTEST01".to_string(),
    };

    let requester = services::snap::request_access_token::AccessTokenRequester::new(&config);

    let response = requester
        .request_access_token()
        .await
        .expect("Failed to get response");

    assert!(response.status().is_success());
}
