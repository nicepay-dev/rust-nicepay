use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InquiryPayoutV2Model {
    time_stamp: String,
    t_xid: String,
    i_mid: String,
    account_no: String,
    merchant_token: String,
}
