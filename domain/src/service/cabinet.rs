use crate::entity::cabinet::{Cabinet, CabinetItem, CabinetStatus, CabinetUsage};
use crate::error::DomainError;
use crate::error::cabinet::CabinetError;
use crate::repository::cabinet::{CabinetItemRepository, CabinetRepository};
use chrono::Local;

pub struct CabinetService<CR, CIR>
where
    CR: CabinetRepository,
    CIR: CabinetItemRepository,
{
    cabinet_repository: CR,
    cabinet_item_repository: CIR,
    cabinets_number: u64,
}

impl<CR, CIR> CabinetService<CR, CIR>
where
    CR: CabinetRepository,
    CIR: CabinetItemRepository,
{
    pub fn new(cabinet_repository: CR, cabinet_item_repository: CIR, cabinets_number: u64) -> Self {
        Self {
            cabinet_repository,
            cabinet_item_repository,
            cabinets_number,
        }
    }
}

impl<CR, CIR> CabinetService<CR, CIR>
where
    CR: CabinetRepository,
    CIR: CabinetItemRepository,
{
    /// Apply for a cabinet
    pub async fn apply(&self) -> Result<Cabinet, DomainError> {
        let now = Local::now();
        let deleted_count = self.cabinet_repository.delete_expired(now).await?;
        log::info!("Deleted {deleted_count} expired cabinets");
        let used = self
            .cabinet_repository
            .count_by_status(CabinetStatus::Occupied)
            .await?;
        if used >= self.cabinets_number {
            return Err(CabinetError::NoAvailableCabinet)?;
        }

        loop {
            let code = rand::random_range(100000..999999);
            let exists = self.cabinet_repository.exists_by_code(code).await?;
            log::debug!("Trying to apply for a cabinet with code '{code}'");
            if !exists {
                let hold_token = uuid::Uuid::new_v4().simple().to_string();
                let expire_at = Local::now() + chrono::Duration::minutes(10);
                let cabinet = Cabinet::new(
                    code,
                    None,
                    None,
                    CabinetStatus::Hold,
                    Some(hold_token),
                    Some(expire_at),
                );
                let cabinet = self.cabinet_repository.save(cabinet.clone()).await?;
                log::info!("Applied for a cabinet with code '{code}'");
                return Ok(cabinet);
            } else {
                log::debug!("Cabinet with code '{code}' already exists");
            }
        }
    }

    /// Save items
    pub async fn save(
        &self,
        cabinet: Cabinet,
        items: Vec<CabinetItem>,
    ) -> Result<Cabinet, DomainError> {
        // Check params
        if cabinet.password.is_none() {
            return Err(CabinetError::PasswordRequired)?;
        }
        if cabinet.expire_at.is_none() {
            return Err(CabinetError::ExpireTimeRequired)?;
        }
        if cabinet.hold_token.is_none() {
            return Err(CabinetError::HoldTokenRequired)?;
        }
        let exists_cabinet = self.cabinet_repository.find_by_code(cabinet.code).await?;
        if exists_cabinet.is_none() {
            return Err(CabinetError::NotFound)?;
        }
        let mut exists_cabinet = exists_cabinet.unwrap();

        // Check status
        let is_hold = exists_cabinet.status == CabinetStatus::Hold;
        let is_your_hold =
            exists_cabinet.hold_token.is_some() && exists_cabinet.hold_token == cabinet.hold_token;
        if !is_hold || !is_your_hold {
            log::error!(
                "Cabinet '{}' is not hold by {:?}: status: {}, token: {:?}",
                exists_cabinet.code,
                cabinet.hold_token,
                exists_cabinet.status,
                exists_cabinet.hold_token,
            );
            return Err(CabinetError::NotYourHoldCabinet(cabinet.code))?;
        }

        // Update cabinet
        exists_cabinet.status = CabinetStatus::Occupied;
        exists_cabinet.hold_token = None;
        exists_cabinet.name = cabinet.name;
        exists_cabinet.description = cabinet.description;
        exists_cabinet.password = cabinet.password;
        exists_cabinet.expire_at = cabinet.expire_at;
        self.cabinet_repository
            .update_by_code(exists_cabinet.clone())
            .await?;

        // Save cabinet items
        let item_size = items.len();
        for item in items {
            self.cabinet_item_repository.save(item).await?;
        }
        log::info! {"Cabinet '{}' locked with {} items.", cabinet.code, item_size};
        Ok(exists_cabinet)
    }

    /// Delete cabinet and items by code
    pub async fn delete_by_code(&self, cabinet_code: i64) -> Result<(), DomainError> {
        let exists = self.cabinet_repository.exists_by_code(cabinet_code).await?;
        if !exists {
            return Err(CabinetError::NotFound)?;
        }
        log::info!("Delete cabinet '{cabinet_code}'");
        self.cabinet_repository.delete_by_code(cabinet_code).await?;

        let items = self
            .cabinet_item_repository
            .list_by_cabinet_code(cabinet_code)
            .await?;
        for item in items {
            log::debug!(
                "Delete cabinet item by id '{}': category is '{}', name is '{}'",
                item.id,
                item.category,
                item.name
            );
            self.cabinet_item_repository.delete_by_id(item.id).await?;
        }
        Ok(())
    }

    /// Get cabinet by code
    pub async fn get_by_code(&self, code: i64) -> Result<Option<Cabinet>, DomainError> {
        self.cabinet_repository.find_by_code(code).await
    }

    /// Get non-none cabinet by code
    pub async fn get_nonnone_by_code(&self, code: i64) -> Result<Cabinet, DomainError> {
        let cabinet = self.cabinet_repository.find_by_code(code).await?;
        if cabinet.is_none() {
            return Err(CabinetError::NotFound)?;
        }
        Ok(cabinet.unwrap())
    }

    /// Get the status of the cabinets
    pub async fn usage(&self) -> Result<CabinetUsage, DomainError> {
        let used = self
            .cabinet_repository
            .count_by_status(CabinetStatus::Occupied)
            .await?;
        Ok(CabinetUsage::new(self.cabinets_number, used))
    }

    /// Get all the items in a cabinet
    pub async fn list_items_by_cabinet_code(
        &self,
        code: i64,
    ) -> Result<Vec<CabinetItem>, DomainError> {
        let exists = self.cabinet_repository.exists_by_code(code).await?;
        if !exists {
            return Err(CabinetError::NotFound)?;
        }
        self.cabinet_item_repository
            .list_by_cabinet_code(code)
            .await
    }

    /// Get a cabinet item by id
    pub async fn get_item_by_id(
        &self,
        item_id: i64,
        with_content: bool,
    ) -> Result<Option<CabinetItem>, DomainError> {
        self.cabinet_item_repository
            .find_by_id(item_id, with_content)
            .await
    }
}
