// Bring in the needed crates and modules
use rust_nicepay::services::v1::request_direct::DirectV1Requester;
use rust_nicepay::models::v1::inquiry_direct_model::InquiryDirectV1Model;
use rust_nicepay::utils::{config::Config, helper::Helper};
use tokio;

#[tokio::test]
async fn test_request_check_status() {
    // Config setup
    let config = Config {
        merchant_key: "33F49GnCMS1mFYlGXisbUDzVf2ATWCl9k3R++d5hDd3Frmuos/XLx8XhXpe+LDYAbpGKZYSwtlyyLOtS/8aD7A==".to_string(),
        is_cloud_server: false,
        is_production: false,
        private_key: "".to_string(),
        client_secret: "".to_string(),
        client_id: "".to_string(),
        channel_id: "".to_string(),
    };

    // test data
    let i_mid = "IONPAYTEST";
    let amt = "10000";
    let reference_no = "REF123456";

    // Generate merchant token
    let merchant_token = Helper::generate_merchant_token(i_mid, reference_no, amt, &config.merchant_key);

let req_check_status: InquiryDirectV1Model = InquiryDirectV1Model {
    // Required fields
    i_mid: i_mid.to_string(),
    t_xid: "IONPAYTEST02202511141416318952".to_string(),
    reference_no: "REF123456".to_string(),
    amt: amt.to_string(),
    merchant_token: merchant_token,
};

    // Create requester
    let requester = DirectV1Requester::new(&config);
    
    // Send the request
    let response = requester
        .request_inquiry(req_check_status)
        .await
        .expect("Failed to send request");

    assert!(
        response.status().is_success(),
        "Request failed with status: {:?}",
        response.status()
    );

    let body = response.text().await.expect("Failed to read response body");
    println!("Response: {}", body);

    assert!(
        body.contains("resultCd") || body.contains("tid"),
        "Response did not contain expected fields"
    );
}

