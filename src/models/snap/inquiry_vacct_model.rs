use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InquiryVacctModel {
    pub partner_service_id: String,
    pub customer_no: String,
    pub virtual_account_no: String,
    pub inquiry_request_id: String,
    pub additional_info: AdditionalInfo,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalInfo {
    #[serde(rename = "trxId")]
    pub trx_id: String,
    #[serde(rename = "tXidVA")]
    pub txid_va: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: TotalAmount,
}

#[derive(Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: String,
    pub currency: String,
}
