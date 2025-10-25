use chrono::{DateTime, Local};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CryptoKeypair {
    pub id: Option<uuid::Uuid>,
    pub secret_key: String,
    pub public_key: String,
    pub expire_at: DateTime<Local>,
    pub create_at: Option<DateTime<Local>>,
    pub update_at: Option<DateTime<Local>>,
    pub version: Option<i32>,
}

impl CryptoKeypair {
    pub fn new(secret_key: String, public_key: String, expire_at: DateTime<Local>) -> Self {
        CryptoKeypair {
            id: None,
            secret_key,
            public_key,
            expire_at,
            create_at: None,
            update_at: None,
            version: None,
        }
    }
}
