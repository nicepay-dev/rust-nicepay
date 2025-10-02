use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistPayoutModel {
    pub partner_reference_no: String,
    pub merchant_id: String,
    pub ms_id: String,
    pub beneficiary_account_no: String,
    pub beneficiary_name: String,
    pub beneficiary_phone: String,
    pub beneficiary_customer_residence: String,
    pub beneficiary_customer_type: String,
    pub beneficiary_postal_code: String,
    pub payout_method: String,
    pub beneficiary_bank_code: String,
    pub description: String,
    pub reserved_dt: String,
    pub reserved_tm: String,
    pub amount: Amount,
}

#[derive(Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}
