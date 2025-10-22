use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::entity::cabinet_item::{ActiveModel, Column, Entity, Model};
use domain::entity::cabinet::{CabinetItem, CabinetItemCategory};
use domain::error::DomainError;
use domain::repository::cabinet::CabinetItemRepository as Repository;
use sea_orm::{QueryOrder, prelude::*};

pub struct CabinetItemRepository {
    connection: DatabaseConnection,
    store_folder: PathBuf,
}

impl CabinetItemRepository {
    pub fn new(connection: DatabaseConnection, store_folder: PathBuf) -> Self {
        Self {
            connection,
            store_folder,
        }
    }
}

impl CabinetItemRepository {
    fn resolve_file_path(&self, cabinet_code: i64, cabinet_item_id: i64) -> PathBuf {
        self.store_folder
            .join(cabinet_code.to_string())
            .join(cabinet_item_id.to_string())
    }

    /// Write content to filesystem
    fn write_content(&self, path: &Path, content: &[u8]) -> Result<(), DomainError> {
        if let Err(e) = std::fs::write(path, content) {
            log::error!("Failed to write cabinet content to '{:?}': {}", path, e);
            return Err(DomainError::InternalError);
        }
        Ok(())
    }

    /// Read content from filesystem
    fn read_content(&self, path: &Path) -> Result<Vec<u8>, DomainError> {
        let content = std::fs::read(path);
        if let Err(e) = content {
            log::error!("Failed to read cabinet content from '{:?}': {}", path, e);
            return Err(DomainError::InternalError);
        }
        Ok(content.unwrap())
    }
}

#[async_trait::async_trait]
impl Repository for CabinetItemRepository {
    async fn save(&self, cabinet_item: CabinetItem) -> Result<(), DomainError> {
        let content = if let Some(content) = cabinet_item.content.as_ref() {
            content
        } else {
            return Err(DomainError::CabinetItemContentMustNotEmpty);
        };

        // Write content to filesystem
        let path = self.resolve_file_path(cabinet_item.cabinet_code, cabinet_item.id);
        self.write_content(&path, content)?;

        // Save cabinet item to database
        let mut model = Model::try_from(cabinet_item)?;
        model.path = Some(path.to_string_lossy().to_string());
        let active_model = ActiveModel::from(model);
        active_model.insert(&self.connection).await.map_err(|e| {
            log::error!("Failed to save cabinet item: {}", e);
            DomainError::InternalError
        })?;
        Ok(())
    }

    async fn delete_by_id(&self, id: i64) -> Result<(), DomainError> {
        Entity::delete_by_id(id)
            .exec(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to delete cabinet item '{}': {}", id, e);
                DomainError::InternalError
            })?;
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: i64,
        with_content: bool,
    ) -> Result<Option<CabinetItem>, DomainError> {
        let model = Entity::find_by_id(id)
            .one(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to find cabinet item '{}': {}", id, e);
                DomainError::InternalError
            })?;

        if model.is_none() {
            return Ok(None);
        }
        let model = model.unwrap();

        let path = PathBuf::from(model.path.as_ref().unwrap());
        let mut cabinet_item = CabinetItem::try_from(model)?;
        if with_content {
            let content = self.read_content(&path)?;
            cabinet_item.content = Some(content);
        }
        Ok(Some(cabinet_item))
    }

    async fn list_by_cabinet_code(
        &self,
        cabinet_code: i64,
    ) -> Result<Vec<CabinetItem>, DomainError> {
        let models = Entity::find()
            .filter(Column::CabinetCode.eq(cabinet_code))
            .order_by_asc(Column::SortOrder)
            .all(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to list cabinet items: {}", e);
                DomainError::InternalError
            })?;
        let mut cabinet_items = Vec::with_capacity(models.len());
        for model in models {
            let cabinet_item = CabinetItem::try_from(model)?;
            cabinet_items.push(cabinet_item);
        }
        Ok(cabinet_items)
    }
}

impl TryFrom<Model> for CabinetItem {
    type Error = DomainError;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            cabinet_code: value.cabinet_code,
            name: value.name,
            category: CabinetItemCategory::from_str(&value.category)?,
            content: None,
            sort_order: value.sort_order,
        })
    }
}

impl TryFrom<CabinetItem> for Model {
    type Error = DomainError;
    fn try_from(value: CabinetItem) -> Result<Self, Self::Error> {
        let now = chrono::Local::now().naive_local();
        Ok(Model {
            id: value.id,
            cabinet_code: value.cabinet_code,
            category: value.category.to_string(),
            name: value.name,
            path: None,
            sort_order: value.sort_order,
            create_at: now,
            update_at: now,
            version: 1,
        })
    }
}
