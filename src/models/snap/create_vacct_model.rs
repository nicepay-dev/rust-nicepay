use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVacctModel {
    pub partner_service_id: Option<String>,
    pub customer_no: Option<String>,
    pub virtual_account_no: Option<String>,
    pub virtual_account_name: Option<String>,
    pub trx_id: Option<String>,
    pub total_amount: Option<TotalAmount>,
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
    pub bank_cd: String,
    pub goods_nm: String,
    pub db_process_url: String,
    pub vacct_valid_dt: String,
    pub vacct_valid_tm: String,
    pub ms_id: String,
    pub ms_fee: String,
    pub ms_fee_type: String,
    pub mb_fee: String,
    pub mb_fee_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: String,
    pub currency: String,
}
