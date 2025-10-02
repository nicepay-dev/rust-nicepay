use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RejectPayoutModel {
    pub original_reference_no: String,
    pub merchant_id: String,
    pub original_partner_reference_no: String,
}
