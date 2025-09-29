use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStatusDirectDebitModel {
    pub original_reference_no: String,
    pub service_code: String,
    pub merchant_id: String,
    pub sub_merchant_id: String,
    pub original_partner_reference_no: String,
    pub transaction_date: String,
    pub external_store_id: String,
    pub amount: Amount,
    pub additional_info: AdditionalInfo,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalInfo {}

#[derive(Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}
