// Bring in the needed crates and modules
use rust_nicepay::services::v1::request_direct::DirectV1Requester;
use rust_nicepay::models::v1::cancel_direct_model::CancelDirectV1Model;
use rust_nicepay::utils::{config::Config, helper::Helper};
use tokio;

#[tokio::test]
async fn test_request_cancel() {
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
    let i_mid = "NORMALTEST";
    let amt = "10000";
    let t_xid = "NORMALTEST02202511181034498797";
    let cancel_type = "1";

    // Generate merchant token
    let merchant_token = Helper::generate_merchant_token(i_mid, t_xid, amt, &config.merchant_key);

let req_cancel: CancelDirectV1Model = CancelDirectV1Model {
    // Required fields
    i_mid: i_mid.to_string(),
    t_xid: t_xid.to_string(),
    pay_method : "02".to_string(),
    cancel_type: cancel_type.to_string(),
    amt: amt.to_string(),
    merchant_token: merchant_token,
    cancel_msg : None,
    cancel_user_id : None,
    cancel_user_ip : None,
    cancel_user_info : None,
    cancel_retry_cnt : None,
    worker : None,

};

    // Create requester
    let requester = DirectV1Requester::new(&config);
    println!("\n=== Requester ==="); 
    println!("{:?}", req_cancel);
    
    // Send the request
    let response = requester
        .request_cancel(req_cancel)
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

