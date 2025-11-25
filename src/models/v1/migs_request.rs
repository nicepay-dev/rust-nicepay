use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MigsRequest {
    pub instmnt_type: String,
    pub instmnt_mon: String,  
    pub reference_no: String,
    pub card_cvv: String,
    pub call_back_url: String,
    pub one_pass_token: String,     
}
