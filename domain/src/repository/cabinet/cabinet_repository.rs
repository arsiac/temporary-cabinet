use crate::entity::cabinet::{Cabinet, CabinetStatus};
use crate::error::DomainError;
use chrono::{DateTime, Local};

#[async_trait::async_trait]
pub trait CabinetRepository {
    /// Save a cabinet
    async fn save(&self, cabinet: Cabinet) -> Result<Cabinet, DomainError>;

    /// Delete a cabinet by code
    async fn delete_by_code(&self, code: i64) -> Result<(), DomainError>;

    /// Delete expired cabinets
    async fn delete_expired(&self, time: DateTime<Local>) -> Result<u64, DomainError>;

    /// Update a cabinet by code
    async fn update_by_code(&self, cabinet: Cabinet) -> Result<(), DomainError>;

    /// Get the number of cabinets
    async fn count(&self) -> Result<u64, DomainError>;

    /// Get the number of cabinets with the given status
    async fn count_by_status(&self, status: CabinetStatus) -> Result<u64, DomainError>;

    /// Check whether a cabinet with the given code exists.
    async fn exists_by_code(&self, code: i64) -> Result<bool, DomainError>;

    /// Find cabinet by code
    async fn find_by_code(&self, code: i64) -> Result<Option<Cabinet>, DomainError>;
}
