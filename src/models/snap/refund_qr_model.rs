use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundQRModel {
    pub original_reference_no: String,
    pub reason: String,
    pub merchant_id: String,
    pub original_partner_reference_no: String,
    pub external_store_id: String,
    pub partner_refund_no: String,
    pub refund_amount: RefundAmount,
    pub additional_info: AdditionalInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
    pub cancel_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefundAmount {
    pub value: String,
    pub currency: String,
}
