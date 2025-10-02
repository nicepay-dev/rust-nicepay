use base64::Engine;
use base64::engine::general_purpose;
use chrono::{DateTime, Local};
use ring::{
    hmac, rand,
    signature::{self, RsaKeyPair},
};
use serde::Serialize;
use serde_json;
use sha2::{Digest, Sha256};

pub struct Helper;

impl Helper {
    pub fn sign_with_private_key_string(
        private_key_str: &str,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let key_bytes = general_purpose::STANDARD
            .decode(private_key_str)
            .map_err(|e| format!("Base64 decode error: {}", e))?;

        let key_pair =
            RsaKeyPair::from_pkcs8(&key_bytes).map_err(|e| format!("PKCS8 parse error: {}", e))?;

        let rng = rand::SystemRandom::new();
        let mut signature = vec![0; key_pair.public().modulus_len()];

        key_pair
            .sign(
                &signature::RSA_PKCS1_SHA256,
                &rng,
                message.as_bytes(),
                &mut signature,
            )
            .map_err(|e| format!("Signing error: {}", e))?;

        Ok(general_purpose::STANDARD.encode(&signature))
    }

    pub fn get_formatted_date() -> String {
        let local_time: DateTime<Local> = Local::now();
        local_time.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
    }

    pub fn get_timestamp_old_format() -> String {
        let now = Local::now();
        now.format("%Y%m%d%H%M%S").to_string()
    }

    pub fn hex_encoded_payload<T: Serialize>(
        data: &T,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let minified = serde_json::to_string(data)?;
        let hash = Sha256::digest(minified.as_bytes());
        let hex_encode = hex::encode(hash).to_lowercase();
        Ok(hex_encode)
    }

    pub fn hash_with_hmac256(
        client_secret: &str,
        string_to_sign: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let key = hmac::Key::new(hmac::HMAC_SHA512, client_secret.as_bytes());
        let signature = hmac::sign(&key, string_to_sign.as_bytes());

        Ok(general_purpose::STANDARD.encode(signature))
    }
}
