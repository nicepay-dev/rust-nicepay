use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InquiryDirectV1Model {
    pub t_xid: String,
    pub i_mid: String,
    pub merchant_token: String,
    pub reference_no: String,
    pub amt: String,
}
