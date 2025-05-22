use std::fs;

use super::base64url::encode_b64::converter_json_base64;
use super::rsa::encrypt_rsa::encrypt;

use super::rsa::generate_keys::generate_keys;
use super::structs::ChatMessage;
use super::structs::Header;

pub fn generate_jwt(
    payload: &ChatMessage
) -> Result<String, String> {

    generate_keys();

    let content = fs::read_to_string("src/key.txt")
    .expect("Erro ao ler o arquivo");
    
    let mut lines = content.lines();

    let n = lines.next().unwrap().parse::<u64>().unwrap();
    let e = lines.next().unwrap().parse::<u64>().unwrap();

    let header = Header {
        alg: "HS256".to_string(),
        typ: "JWT".to_string(),
        n: n,
        e: e,
    };

    let header_json = serde_json::to_string(&header).map_err(|e| e.to_string())?;
    let header_b64 = converter_json_base64(&header_json);

    let payload_json = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let payload_b64 = converter_json_base64(&payload_json);

    let signature = format!("{}.{}", header_b64, payload_b64);
    let signature_rsa = encrypt(signature.as_bytes(), &n, &e);
    let signature_b64 = converter_json_base64(&signature_rsa);

    Ok(format!("{}.{}.{}", header_b64, payload_b64, signature_b64))
}
