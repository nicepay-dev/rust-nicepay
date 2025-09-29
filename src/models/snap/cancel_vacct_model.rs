use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelVacctModel {
    pub partner_service_id: String,
    pub customer_no: String,
    pub virtual_account_no: String,
    pub trx_id: String,
    pub additional_info: AdditionalInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
    #[serde(rename = "tXidVA")]
    pub txid_va: String,
    #[serde(rename = "cancelMessage")]
    pub cancel_message: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: TotalAmount,
}

#[derive(Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: String,
    pub currency: String,
}
