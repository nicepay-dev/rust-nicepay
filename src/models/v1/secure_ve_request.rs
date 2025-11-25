use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecureVeRequest {
    pub country: String,
    pub call_back_url: String,  
    pub one_pass_token: String, 
}
