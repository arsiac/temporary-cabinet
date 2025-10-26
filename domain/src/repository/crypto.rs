use crate::entity::crypto::CryptoKeypair;
use crate::error::DomainError;
use chrono::{DateTime, Local};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait CryptoKeypairRepository {
    /// Save a keypair.
    async fn save(&self, keypair: CryptoKeypair) -> Result<CryptoKeypair, DomainError>;

    /// Delete a keypair by id.
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DomainError>;

    /// Delete expired keypairs.
    async fn delete_expired(&self, time: DateTime<Local>) -> Result<u64, DomainError>;

    /// Count all keypairs.
    async fn count(&self) -> Result<u64, DomainError>;

    /// Find a keypair by public key.
    async fn find_by_public_key(
        &self,
        public_key: &str,
    ) -> Result<Option<CryptoKeypair>, DomainError>;
}
