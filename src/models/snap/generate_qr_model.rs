use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateQRModel {
    pub partner_reference_no: String,
    pub validity_period: String,
    pub store_id: String,
    pub merchant_id: String,
    pub amount: Amount,
    pub additional_info: AdditionalInfo,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalInfo {
    #[serde(rename = "goodsNm")]
    pub goods_nm: String,
    #[serde(rename = "dbProcessUrl")]
    pub db_process_url: String,
    #[serde(rename = "billingNm")]
    pub billing_nm: String,
    #[serde(rename = "billingPhone")]
    pub billing_phone: String,
    #[serde(rename = "billingEmail")]
    pub billing_email: String,
    #[serde(rename = "billingCity")]
    pub billing_city: String,
    #[serde(rename = "billingAddr")]
    pub billing_addr: String,
    #[serde(rename = "billingState")]
    pub billing_state: String,
    #[serde(rename = "billingPostCd")]
    pub billing_post_cd: String,
    #[serde(rename = "billingCountry")]
    pub billing_country: String,
    #[serde(rename = "userIP")]
    pub user_ip: String,
    #[serde(rename = "cartData")]
    pub cart_data: String,
    #[serde(rename = "mitraCd")]
    pub mitra_cd: String,
    #[serde(rename = "msId")]
    pub ms_id: String,
    #[serde(rename = "msFee")]
    pub ms_fee: String,
    #[serde(rename = "msFeeType")]
    pub ms_fee_type: String,
    #[serde(rename = "mbFee")]
    pub mb_fee: String,
    #[serde(rename = "mbFeeType")]
    pub mb_fee_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}
