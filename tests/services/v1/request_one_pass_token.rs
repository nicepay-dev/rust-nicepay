use rust_nicepay::services::v1::request_direct::DirectV1Requester;
use rust_nicepay::models::v1::one_pass_token::OnePassTokenJsonData;
use rust_nicepay::utils::{config::Config, helper::Helper};

#[tokio::test]
async fn test_request_one_pass_token_success() {
    let config = Config {
        merchant_key: "33F49GnCMS1mFYlGXisbUDzVf2ATWCl9k3R++d5hDd3Frmuos/XLx8XhXpe+LDYAbpGKZYSwtlyyLOtS/8aD7A==".to_string(),
        is_cloud_server: false,
        is_production: true,
        private_key: "".to_string(),
        client_secret: "".to_string(),
        client_id: "".to_string(),
        channel_id: "".to_string(),
    };

    let i_mid = "TNICECC015";
    let reference_no = "ONEPASS12345";
    let amt = "1000";

    // Generate merchant token
    let merchant_token =
        Helper::generate_merchant_token(i_mid, reference_no, amt, &config.merchant_key);

    // === Build Request Model ===
    let req = OnePassTokenJsonData {
        i_mid: i_mid.to_string(),
        reference_no: reference_no.to_string(),
        amt: amt.to_string(),
        card_holder_nm: "John Doe".to_string(),
        card_holder_email: "john@example.com".to_string(),
        card_no: "4889506048042950".to_string(),
        card_exp_yymm: "2703".to_string(),
        merchant_token: merchant_token,
    };

    // Create requester
    let requester = DirectV1Requester::new(&config);

    // === Send Request ===
    let response = requester
        .request_one_pass_token(req)
        .await
        .expect("Request failed");

    assert!(
        response.status().is_success(),
        "Status not OK: {:?}",
        response.status()
    );

    let body = response.text().await.expect("Failed to read body");
    println!("OnePassToken Response: {}", body);

    // Basic validation
    assert!(
        body.contains("resultCd") || body.contains("tXid"),
        "Unexpected response body"
    );
}
