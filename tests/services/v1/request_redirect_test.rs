use rust_nicepay::services::v1::request_redirect::RedirectV1Requester;
use rust_nicepay::models::v1::regist_redirect_model::RegistRedirectV1Model;
use rust_nicepay::utils::{config::Config, helper::Helper};
use tokio;

#[tokio::test]
async fn test_request_regist_success() {
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

let req_redirect: RegistRedirectV1Model = RegistRedirectV1Model {
    // Required fields
    merchant_token: merchant_token,
    pay_method: "03".to_string(),
    currency: "IDR".to_string(),
    mitra_cd: "ALMA".to_string(),
    amt: amt.to_string(),
    reference_no: reference_no.to_string(),
    goods_nm: "Goods".to_string(),
    call_back_url: "https://www.nicepay.co.id/IONPAY_CLIENT/paymentResult.jsp".to_string(),
    db_process_url: "https://ptsv2.com/t/51bedu1/post".to_string(),
    billing_nm: "Hantu Kesorean".to_string(),
    billing_phone: "081288998899".to_string(),
    user_ip: "127.0.0.1".to_string(),
    i_mid: i_mid.to_string(),
    billing_email: "abdul@example.com".to_string(),
    billing_addr: "Jl. Example".to_string(),
    billing_city: "Jakarta Selatan".to_string(),
    billing_state: "DKI Jakarta".to_string(),
    billing_post_cd: "12345".to_string(),
    billing_country: "Indonesia".to_string(),
    description: "testing CVS V1".to_string(),
    
    // Optional fields
    time_stamp: None,
    cart_data: None,
    instmnt_type: None,
    instmnt_mon: None,
    recurr_opt: None,
    user_session_i_d: None,
    user_agent: None,
    user_language: None,
    bank_cd: None,
    vacct_valid_dt: None,
    vacct_valid_tm: None,
    payment_exp_dt: None,
    payment_exp_tm: None,
    pay_valid_dt: None,
    pay_valid_tm: None,
    mer_fix_acct_id: None,
    delivery_nm: None,
    delivery_phone: None,
    delivery_addr: None,
    delivery_city: None,
    delivery_state: None,
    delivery_post_cd: None,
    delivery_country: None,
    req_domain: None,
    req_server_i_p: None,
    req_client_ver: None,
    req_dt: None,
    req_tm: None,
    sellers: None,
    shop_id: None,
    vat: None,
    fee: None,
    notax_amt: None,
    ms_id: None,
    ms_fee: None,
    ms_fee_type: None,
    mb_fee: None,
    mb_fee_type: None,
};

    // Create requester
    let requester = RedirectV1Requester::new(&config);

    // Send the request
    let response = requester
        .request_regist(req_redirect)
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

