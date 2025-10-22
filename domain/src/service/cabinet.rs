use crate::entity::cabinet::{Cabinet, CabinetItem, CabinetItemCategory};
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
    pub async fn initialize(&self, cabinet_number: u64) -> Result<(), DomainError> {
        log::info!("Initializing cabinets({})...", cabinet_number);
        let current_number = self.cabinet_repository.count().await?;
        if current_number == cabinet_number {
            log::info!("Cabinets already initialized'");
            return Ok(());
        }

        if current_number < cabinet_number {
            for i in current_number + 1..=cabinet_number {
                let cabinet = Cabinet::new(i as i64, None, None);
                self.cabinet_repository.save(cabinet).await?;
            }
            log::info!(
                "Cabinets [{}, {}] initialized",
                current_number + 1,
                cabinet_number
            );
            return Ok(());
        }

        if current_number > cabinet_number {
            let codes: Vec<i64> = (cabinet_number + 1..=current_number)
                .map(|e| e as i64)
                .collect();
            self.cabinet_repository
                .update_pending_destruction_by_codes(codes, true)
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
}
