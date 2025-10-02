use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryQRModel {
    pub original_reference_no: String,
    pub service_code: String,
    pub merchant_id: String,
    pub original_partner_reference_no: String,
    pub external_store_id: String,
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalInfo {}
