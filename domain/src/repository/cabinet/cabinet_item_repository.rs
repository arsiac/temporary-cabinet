use crate::entity::cabinet::CabinetItem;
use crate::error::DomainError;

#[async_trait::async_trait]
pub trait CabinetItemRepository {
    /// Save cabinet item
    async fn save(&self, cabinet_item: CabinetItem) -> Result<(), DomainError>;

    /// Delete cabinet item by id
    async fn delete_by_id(&self, id: i64) -> Result<(), DomainError>;

    /// Find cabinet item by id
    async fn find_by_id(
        &self,
        id: i64,
        with_content: bool,
    ) -> Result<Option<CabinetItem>, DomainError>;

    /// Get cabinet item by cabinet code
    async fn list_by_cabinet_code(
        &self,
        cabinet_code: i64,
    ) -> Result<Vec<CabinetItem>, DomainError>;
}
