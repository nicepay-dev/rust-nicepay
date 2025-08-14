use chrono::{DateTime, Local, TimeZone, Utc};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use openssl::sha::Sha256;
use openssl::sign::{Signer, Verifier};
use serde_json::Value;
use std::fmt::Write;

pub struct Helper;

impl Helper {
    pub fn get_signature_access_token(
        private_key_string: &str,
        string_to_sign: &str,
    ) -> Result<String, openssl::error::ErrorStack> {
        let private_key = PKey::private_key_from_pem(private_key_string.as_bytes())?;
        let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
        signer.update(string_to_sign.as_bytes())?;
        let signature = signer.sign_to_vec()?;
        Ok(base64::encode(signature))
    }

    pub fn get_encode_payload(request_body: &Value) -> String {
        let string = request_body.to_string();
        let mut hasher = Sha256::new();
        hasher.update(string.as_bytes());
        hex::encode(hasher.finish())
    }

    pub fn get_regist_signature(
        string_to_sign: &str,
        client_secret: &str,
    ) -> Result<String, openssl::error::ErrorStack> {
        let key = PKey::hmac(client_secret.as_bytes())?;
        let mut signer = Signer::new(MessageDigest::sha512(), &key)?;
        signer.update(string_to_sign.as_bytes())?;
        let hmac = signer.sign_to_vec()?;
        Ok(base64::encode(hmac))
    }

    pub fn get_formatted_date() -> String {
        let local = Local::now();
        local.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
    }

    pub fn get_convert_formatted_date(time_stamp: &str) -> String {
        if time_stamp.len() < 14 {
            return String::new();
        }

        let year = &time_stamp[0..4];
        let month = &time_stamp[4..6];
        let day = &time_stamp[6..8];
        let hour = &time_stamp[8..10];
        let minute = &time_stamp[10..12];
        let second = &time_stamp[12..14];
        let offset = "+07:00";

        format!(
            "{}-{}-{}T{}:{}:{}{}",
            year, month, day, hour, minute, second, offset
        )
    }

    pub fn verify_sha256_rsa(
        string_to_sign: &str,
        public_key_string: &str,
        signature_string: &str,
    ) -> bool {
        let pem_key = Self::format_pem_key(public_key_string);

        let public_key = match PKey::public_key_from_pem(pem_key.as_bytes()) {
            Ok(key) => key,
            Err(e) => {
                eprintln!("Error creating public key: {}", e);
                return false;
            }
        };

        let signature = match base64::decode(signature_string) {
            Ok(sig) => sig,
            Err(e) => {
                eprintln!("Error decoding signature: {}", e);
                return false;
            }
        };

        let mut verifier = match Verifier::new(MessageDigest::sha256(), &public_key) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error creating verifier: {}", e);
                return false;
            }
        };

        verifier.update(string_to_sign.as_bytes()).unwrap();
        match verifier.verify(&signature) {
            Ok(valid) => valid,
            Err(e) => {
                eprintln!("Error verifying signature: {}", e);
                false
            }
        }
    }

    fn format_pem_key(public_key_string: &str) -> String {
        format!(
            "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
            public_key_string
        )
    }

    pub fn generate_merchant_token(string_to_sign: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(string_to_sign.as_bytes());
        hex::encode(hasher.finish())
    }

    pub fn get_timestamp_old_format() -> String {
        let now = Local::now();
        now.format("%Y%m%d%H%M%S").to_string()
    }
}
