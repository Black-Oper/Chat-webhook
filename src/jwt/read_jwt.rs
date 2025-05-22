use super::base64url::decode_b64::{decodificar_json_base64, decodificar_signature};
use super::rsa::encrypt_rsa::encrypt;
use super::structs::Header;
use serde_json::{from_value, Value};

pub fn read_jwt(jwt: &str) -> Result<String, String> {
    // 1. Separa header, payload e assinatura
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT format".into());
    }
    let (header_b64, payload_b64, signature_b64) = (parts[0], parts[1], parts[2]);

    let payload_value: Value = decodificar_json_base64(payload_b64)
        .map_err(|e| e.to_string())?;

    let header_value: Value = decodificar_json_base64(header_b64)
        .map_err(|e| e.to_string())?;

    let header_json: Header = from_value(header_value.clone())
        .map_err(|e| e.to_string())?;

    let signature_rsa = decodificar_signature(signature_b64)
        .map_err(|e| e.to_string())?;
    let signing_input = format!("{}.{}", header_b64, payload_b64);
    let computed_signature =
        encrypt(signing_input.as_bytes(), &header_json.n, &header_json.e);

    if signature_rsa != computed_signature {
        return Err("Invalid JWT signature".into());
    }

    Ok(payload_value.to_string())
}