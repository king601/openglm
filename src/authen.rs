use std::time::{SystemTime, UNIX_EPOCH};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ring::hmac;
use serde_json::json;

use crate::error::{Error, Result};

const EXPIRE_SECOND: i64 = 180;

pub(crate) fn generate(api_key: &str) -> Result<String> {
    let Some((id, secret)) = api_key.split_once(".") else {
        return Err(Error::InvalidApiKey);
    };

    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() as i64;

    _generate(id, secret, timestamp)
}

fn _generate(id: &str, secret: &str, timestamp: i64) -> Result<String> {
    let payload = json!({
        "api_key": id,
        "exp": timestamp + EXPIRE_SECOND * 1000,
        "timestamp": timestamp,
    });

    _encode(payload, secret)
}

fn _encode(payload: serde_json::Value, secret: &str) -> Result<String> {
    let header = json!(
        {
            "alg": "HS256",
            "typ": "JWT",
            "sign_type": "SIGN",
        }
    );

    let encoded_header = b64_encode_part(&header)?;
    let encoded_claims = b64_encode_part(&payload)?;
    let message = [encoded_header, encoded_claims].join(".");
    let signature = sign_hmac(hmac::HMAC_SHA256, secret.as_bytes(), message.as_bytes());

    Ok([message, signature].join("."))
}

fn b64_encode_part<T: serde::Serialize>(input: &T) -> Result<String> {
    let json = serde_json::to_vec(input).unwrap();
    Ok(b64_encode(json))
}

fn sign_hmac(alg: hmac::Algorithm, key: &[u8], message: &[u8]) -> String {
    let digest = hmac::sign(&hmac::Key::new(alg, key), message);
    b64_encode(digest)
}

fn b64_encode<T: AsRef<[u8]>>(input: T) -> String {
    URL_SAFE_NO_PAD.encode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let api_key = "b0bd15d8b10aa938a9bb52faee28772f";
        let secret = "iAPGMaNCsZzruiyK";
        let timestamp = 1711433468000;
        let token = _generate(api_key, secret, timestamp).unwrap();
        assert_eq!(&token, "eyJhbGciOiJIUzI1NiIsInNpZ25fdHlwZSI6IlNJR04iLCJ0eXAiOiJKV1QifQ.eyJhcGlfa2V5IjoiYjBiZDE1ZDhiMTBhYTkzOGE5YmI1MmZhZWUyODc3MmYiLCJleHAiOjE3MTE0MzM2NDgwMDAsInRpbWVzdGFtcCI6MTcxMTQzMzQ2ODAwMH0.cGFi7-UzOt2Wl4T2SyMJ0gIeAiyBJ_iQjcKS38I6f0A");
    }
}