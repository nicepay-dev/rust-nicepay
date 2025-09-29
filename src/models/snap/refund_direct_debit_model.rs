use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundDirectDebitModel {
    pub original_reference_no: String,
    pub merchant_id: String,
    pub sub_merchant_id: String,
    pub original_partner_reference_no: String,
    pub partner_refund_no: String,
    pub external_store_id: String,
    pub reason: String,
    pub refund_amount: RefundAmount,
    pub additional_info: AdditionalInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
    pub refund_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefundAmount {
    pub value: String,
    pub currency: String,
}
