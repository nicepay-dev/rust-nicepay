use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckBalancePayoutModel {
    pub account_no: String,
    pub additional_info: AdditionalInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
    pub ms_id: String,
}
