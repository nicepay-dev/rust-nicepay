use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistRedirectV1Model {
    // Required fields
    pub i_mid: String,
    pub pay_method: String,
    pub currency: String,
    pub amt: String,
    pub reference_no: String,
    pub goods_nm: String,
    pub billing_nm: String,
    pub billing_phone: String,
    pub billing_email: String,
    pub billing_addr: String,
    pub billing_city: String,
    pub billing_state: String,
    pub billing_post_cd: String,
    pub billing_country: String,
    pub db_process_url: String,
    pub call_back_url: String,
    pub user_ip: String,
    pub merchant_token: String,
    pub description: String,
    pub mitra_cd: String,
    
    // Optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_data: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instmnt_type: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instmnt_mon: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurr_opt: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_session_i_d: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_language: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_cd: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vacct_valid_dt: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vacct_valid_tm: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_exp_dt: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_exp_tm: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_valid_dt: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_valid_tm: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mer_fix_acct_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_nm: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_phone: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_addr: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_city: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_state: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_post_cd: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_country: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_domain: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_server_i_p: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_client_ver: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_dt: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_tm: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sellers: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notax_amt: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_fee: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_fee_type: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mb_fee: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mb_fee_type: Option<String>,
}