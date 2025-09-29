use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistPayoutV2Model {
    time_stamp: String,
    i_mid: String,
    payout_method: String,
    amt: String,
    reference_no: String,
    benef_nm: String,
    account_no: String,
    benef_phone: String,
    benef_status: String,
    benef_type: String,
    bank_cd: String,
    reserved_dt: String,
    reserved_tm: String,
    merchant_token: String,
    description: String,
    ms_id: String,
}
