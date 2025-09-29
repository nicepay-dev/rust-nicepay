pub mod models;
pub mod services;
pub mod utils;

use models::snap::create_vacct_model::{AdditionalInfo, CreateVacctModel, TotalAmount};
use services::snap::request_access_token::AccessTokenRequester;
use services::snap::request_vacct::VacctSNAPRequester;

#[tokio::test]
async fn test_request_create_va_success() {
    let access_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJJT05QQVlURVNUIiwiaXNzIjoiTklDRVBBWSIsIm5hbWUiOiJQREpCIiwiZXhwIjoiMjAyNS0wOS0yNlQxMTowMDo1MVoifQ==.Ovm84p2U3vUBUwoJ7sR-JOHS9a2WixTPOezkWiZgk6w=";
    let vacct_requestor = VacctSNAPRequester::new(&config);
    let x_external_id = "12781627175831241291291.2311";

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

    let response_vacct = vacct_requestor
        .request_create_va(req_vacct, access_token, x_external_id)
        .await
        .expect("Failed to get response");

    assert!(response.status().is_success());

    let body = response_vacct
        .text()
        .await
        .expect("Failed to read response body");
}
