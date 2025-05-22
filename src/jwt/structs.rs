use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub alg: String,
    pub typ: String,
    pub n: u64,
    pub e: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub username: String,
    pub text: String,
    pub timestamp: String,
}