use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelDirectV2Model {
    time_stamp: String,
    t_xid: String,
    i_mid: String,
    pay_method: String,
    merchant_token: String,
    amt: String,
    reference_no: String,
    cancel_type: String,
    cancel_msg: String,
    cancel_user_id: String,
    cancel_user_ip: String,
    cancel_server_ip: String,
    cancel_user_info: String,
    cancel_retry_cnt: String,
    worker: String,
}
