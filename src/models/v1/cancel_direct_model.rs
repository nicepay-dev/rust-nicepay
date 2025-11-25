use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelDirectV1Model {
    pub t_xid: String,
    pub i_mid: String,
    pub merchant_token: String,
    pub amt: String,
    pub cancel_type: String,
    pub pay_method: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_msg: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_user_ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_user_info: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_retry_cnt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker: Option<String>,
}
