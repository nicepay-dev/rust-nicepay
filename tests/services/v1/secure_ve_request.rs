use rust_nicepay::services::v1::request_direct::DirectV1Requester;
use rust_nicepay::models::v1::secure_ve_request::SecureVeRequest;
use rust_nicepay::utils::{config::Config, helper::Helper};

#[tokio::test]
async fn test_secure_ve_request_success() {
    let config = Config {
        merchant_key: "".to_string(),
        is_cloud_server: false,
        is_production: true,
        private_key: "".to_string(),
        client_secret: "".to_string(),
        client_id: "".to_string(),
        channel_id: "".to_string(),
    };

    let i_mid = "";
    let reference_no = "";
    let amt = "10000";

    // === Build Request Model ===
    let req = SecureVeRequest {
        country: "360".to_string(),
        call_back_url: "https://dev.nicepay.co.id/IONPAY_CLIENT/paymentResult.jsp".to_string(),  
        one_pass_token: "".to_string(), 
    };

    // Create requester
    let requester = DirectV1Requester::new(&config);

    // === Send Request ===
    let response = requester
        .request_secure_ve(req)
        .await
        .expect("Request failed");

    assert!(
        response.status().is_success(),
        "Status not OK: {:?}",
        response.status()
    );

    let body = response.text().await.expect("Failed to read body");
    println!("Response: {}", body);

    // Basic validation
    assert!(
        body.contains("resultCd") || body.contains("tXid"),
        "Unexpected response body"
    );


}
