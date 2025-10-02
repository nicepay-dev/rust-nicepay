use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDirectV2Model {
    time_stamp: String,
    t_xid: String,
    merchant_token: String,
    call_back_url: String,
    card_no: String,
    card_exp_yymm: String,
    card_cvv: String,
    recurring_token: String,
}
