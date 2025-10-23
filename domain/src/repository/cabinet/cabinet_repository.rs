use crate::entity::cabinet::Cabinet;
use crate::error::DomainError;

#[async_trait::async_trait]
pub trait CabinetRepository {
    /// Save a cabinet
    async fn save(&self, cabinet: Cabinet) -> Result<(), DomainError>;

    /// Delete a cabinet by code
    async fn delete_by_code(&self, code: i64) -> Result<(), DomainError>;

    /// Delete unused pending destruction cabinets
    async fn delete_unused_pending_destruction(&self) -> Result<(), DomainError>;

    /// Update a cabinet by code
    async fn update_by_code(&self, cabinet: Cabinet) -> Result<(), DomainError>;

    /// Mark cabinets as pending destruction by code
    async fn update_pending_destruction_by_codes(
        &self,
        codes: Vec<i64>,
        pending_destruction: bool,
    ) -> Result<(), DomainError>;

    /// Get the number of cabinets
    async fn count(&self) -> Result<u64, DomainError>;

    /// Get the number of used cabinets
    async fn count_used(&self) -> Result<u64, DomainError>;

    /// Check whether a cabinet with the given code exists.
    async fn exists_by_code(&self, code: i64) -> Result<bool, DomainError>;

    /// Get the max code of the cabinets
    async fn max_code(&self) -> Result<Option<i64>, DomainError>;

    /// Find cabinet by code
    async fn find_by_code(&self, code: i64) -> Result<Option<Cabinet>, DomainError>;

    /// Get all the unused cabinets
    async fn list_unused(&self) -> Result<Vec<Cabinet>, DomainError>;
}
