use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InquiryDirectV2Model {
    time_stamp: String,
    t_xid: String,
    i_mid: String,
    merchant_token: String,
    reference_no: String,
    amt: String,
}
