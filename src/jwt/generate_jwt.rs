use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

#[derive(Debug, Serialize, Deserialize)]
struct Header {
    alg: String,
    typ: String,
}

pub fn generate_jwt(
    payload: Value,
) -> Result<String, String> {
    let header = Header {
        alg: "HS256".to_string(),
        typ: "JWT".to_string(),
    };

    let header_json = serde_json::to_string(&header).map_err(|e| e.to_string())?;
    let header_b64 = base64_url::encode(&header_json.as_bytes());

    let payload_json = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let payload_b64 = base64_url::encode(&payload_json.as_bytes());

    let signature = format!("{}.{}", header_b64, payload_b64);
    let signature_rsa = encrypt(signature.as_bytes());
    let signature_b64 = base64_url::encode(&signature_rsa.as_bytes());

    Ok(format!("{}.{}.{}", header_b64, payload_b64, signature_b64))
}

