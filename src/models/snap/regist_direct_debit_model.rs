use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistDirectDebitModel {
    pub partner_reference_no: String,
    pub merchant_id: String,
    pub sub_merchant_id: String,
    pub external_store_id: String,
    pub valid_up_to: String,
    pub point_of_initiation: String,
    pub amount: Amount,
    pub url_param: UrlParam,
    pub additional_info: AdditionalInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
    pub goods_nm: String,
    pub db_process_url: String,
    pub call_back_url: String,
    pub billing_nm: String,
    pub billing_phone: String,
    pub billing_email: String,
    pub cart_data: String,
    pub mitra_cd: String,
    pub ms_id: String,
    pub ms_fee: String,
    pub ms_fee_type: String,
    pub mb_fee: String,
    pub mb_fee_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlParam {
    pub url: String,
    pub r#type: String,
    pub is_deeplink: String,
}
