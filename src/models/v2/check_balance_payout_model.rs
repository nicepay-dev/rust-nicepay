use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckBalanceV2Model {
    time_stamp: String,
    i_mid: String,
    merchant_token: String,
}
