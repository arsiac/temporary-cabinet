use crate::entity::cabinet::{ActiveModel, Column, Entity, Model};
use chrono::{DateTime, Local, TimeZone};
use domain::entity::cabinet::{Cabinet, CabinetStatus};
use domain::error::DomainError;
use domain::repository::cabinet::CabinetRepository as Repository;
use sea_orm::prelude::*;

pub struct CabinetRepository {
    connection: DatabaseConnection,
}

impl CabinetRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl Repository for CabinetRepository {
    async fn save(&self, cabinet: Cabinet) -> Result<Cabinet, DomainError> {
        let model = Model::from(cabinet);
        let active_model = ActiveModel::from(model);
        let model = active_model.insert(&self.connection).await.map_err(|e| {
            log::error!("Failed to insert cabinet: {}", e);
            DomainError::InternalError
        })?;
        Ok(Cabinet::try_from(model)?)
    }

    async fn delete_by_code(&self, code: i64) -> Result<(), DomainError> {
        Entity::delete_by_id(code)
            .exec(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to delete cabinet: {}", e);
                DomainError::InternalError
            })?;
        Ok(())
    }

    async fn delete_expired(&self, time: DateTime<Local>) -> Result<u64, DomainError> {
        let res = Entity::delete_many()
            .filter(Column::ExpireAt.lte(time.naive_local()))
            .exec(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to delete expired cabinets: {}", e);
                DomainError::InternalError
            })?;
        Ok(res.rows_affected)
    }

    async fn update_by_code(&self, cabinet: Cabinet) -> Result<(), DomainError> {
        use sea_orm::ActiveValue;
        let model = Model::from(cabinet);
        let mut active_model = ActiveModel::new();
        active_model.code = ActiveValue::Unchanged(model.code);
        active_model.name = ActiveValue::Set(model.name);
        active_model.description = ActiveValue::Set(model.description);
        active_model.password = ActiveValue::Set(model.password);
        active_model.status = ActiveValue::Set(model.status);
        active_model.hold_token = ActiveValue::Set(model.hold_token);
        active_model.expire_at = ActiveValue::Set(model.expire_at);
        active_model.update_at = ActiveValue::Set(model.update_at);
        active_model.version = ActiveValue::Set(model.version + 1);
        active_model.update(&self.connection).await.map_err(|e| {
            log::error!("Failed to update cabinet: {}", e);
            DomainError::InternalError
        })?;
        Ok(())
    }

    async fn count(&self) -> Result<u64, DomainError> {
        Entity::find().count(&self.connection).await.map_err(|e| {
            log::error!("Failed to count cabinet: {}", e);
            DomainError::InternalError
        })
    }

    async fn count_by_status(&self, status: CabinetStatus) -> Result<u64, DomainError> {
        Entity::find()
            .filter(Column::Status.eq(status.code()))
            .count(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to count cabinet: {}", e);
                DomainError::InternalError
            })
    }

    async fn exists_by_code(&self, code: i64) -> Result<bool, DomainError> {
        let count = Entity::find()
            .filter(Column::Code.eq(code))
            .count(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to check existence of cabinet: {}", e);
                DomainError::InternalError
            })?;
        Ok(count > 0)
    }

    async fn find_by_code(&self, code: i64) -> Result<Option<Cabinet>, DomainError> {
        Entity::find_by_id(code)
            .one(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to find cabinet: {}", e);
                DomainError::InternalError
            })?
            .map(Cabinet::try_from)
            .transpose()
    }
}

impl From<Cabinet> for Model {
    fn from(value: Cabinet) -> Self {
        let now = chrono::Local::now().naive_local();
        Model {
            code: value.code,
            name: value.name,
            description: value.description,
            password: value.password,
            status: value.status.code(),
            hold_token: value.hold_token,
            expire_at: value.expire_at.map(|e| e.naive_local()),
            create_at: value.create_at.map(|e| e.naive_local()).unwrap_or(now),
            update_at: value.update_at.map(|e| e.naive_local()).unwrap_or(now),
            version: value.version.unwrap_or(1),
        }
    }
}

impl TryFrom<Model> for Cabinet {
    type Error = DomainError;
    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(Cabinet {
            code: value.code,
            name: value.name,
            description: value.description,
            password: value.password,
            status: CabinetStatus::try_from(value.status)?,
            hold_token: value.hold_token,
            expire_at: value
                .expire_at
                .map(|e| Local.from_local_datetime(&e).single().unwrap()),
            create_at: Local.from_local_datetime(&value.create_at).single(),
            update_at: Local.from_local_datetime(&value.update_at).single(),
            version: Some(value.version),
        })
    }
}
