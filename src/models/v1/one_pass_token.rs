use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OnePassTokenJsonData {
    pub i_mid: String,
    pub reference_no: String,
    pub amt: String,
    pub card_holder_nm: String,
    pub card_holder_email: String,
    pub card_no: String,
    pub card_exp_yymm: String,
    pub merchant_token: String,
}
