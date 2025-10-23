use crate::entity::cabinet::{Cabinet, CabinetItem, CabinetStatus};
use crate::error::DomainError;
use crate::repository::cabinet::{CabinetItemRepository, CabinetRepository};

pub struct CabinetService<CR, CIR>
where
    CR: CabinetRepository,
    CIR: CabinetItemRepository,
{
    cabinet_repository: CR,
    cabinet_item_repository: CIR,
}

impl<CR, CIR> CabinetService<CR, CIR>
where
    CR: CabinetRepository,
    CIR: CabinetItemRepository,
{
    pub fn new(cabinet_repository: CR, cabinet_item_repository: CIR) -> Self {
        Self {
            cabinet_repository,
            cabinet_item_repository,
        }
    }
}

impl<CR, CIR> CabinetService<CR, CIR>
where
    CR: CabinetRepository,
    CIR: CabinetItemRepository,
{
    /// Create a specified number of cabinets
    pub async fn initialize(&self, cabinet_number: i64) -> Result<(), DomainError> {
        log::info!("Initializing cabinets({})...", cabinet_number);
        let current_number = self.cabinet_repository.max_code().await?.unwrap_or(0);
        if current_number == cabinet_number {
            log::info!("Cabinets already initialized'");
            return Ok(());
        }

        if current_number < cabinet_number {
            for code in current_number + 1..=cabinet_number {
                if let Some(mut cabinet) = self.cabinet_repository.find_by_code(code).await? {
                    cabinet.pending_destruction = false;
                    self.cabinet_repository.update_by_code(cabinet).await?;
                } else {
                    let cabinet = Cabinet::new(code, None, None);
                    self.cabinet_repository.save(cabinet).await?;
                }
            }
            log::info!(
                "Cabinets [{}, {}] initialized",
                current_number + 1,
                cabinet_number
            );
            return Ok(());
        }

        if current_number > cabinet_number {
            let codes: Vec<i64> = (cabinet_number + 1..=current_number).collect();
            self.cabinet_repository
                .update_pending_destruction_by_codes(codes, true)
                .await?;
            self.cabinet_repository
                .delete_unused_pending_destruction()
                .await?;
            log::info!(
                "Cabinets [{}, {}] marked as pending destruction",
                cabinet_number + 1,
                current_number
            );
            return Ok(());
        }

        // Should never happen
        Err(DomainError::InternalError)
    }

    /// Get all the unused cabinets
    pub async fn list_unused_cabinets(&self) -> Result<Vec<Cabinet>, DomainError> {
        self.cabinet_repository.list_unused().await
    }

    /// Get the status of the cabinets
    pub async fn status(&self) -> Result<CabinetStatus, DomainError> {
        let total = self.cabinet_repository.count().await?;
        let used = self.cabinet_repository.count_used().await?;
        Ok(CabinetStatus::new(total, used))
    }

    /// Get all the items in a cabinet
    pub async fn list_items_by_cabinet_code(
        &self,
        code: i64,
    ) -> Result<Vec<CabinetItem>, DomainError> {
        let exists = self.cabinet_repository.exists_by_code(code).await?;
        if !exists {
            return Err(DomainError::CabinetNotFound);
        }
        self.cabinet_item_repository
            .list_by_cabinet_code(code)
            .await
    }
}
