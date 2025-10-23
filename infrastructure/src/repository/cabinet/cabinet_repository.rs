use crate::entity::cabinet::{ActiveModel, Column, Entity, Model};
use chrono::{Local, TimeZone};
use domain::entity::cabinet::Cabinet;
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
    async fn save(&self, cabinet: Cabinet) -> Result<(), DomainError> {
        let model = Model::from(cabinet);
        let active_model = ActiveModel::from(model);
        active_model.insert(&self.connection).await.map_err(|e| {
            log::error!("Failed to insert cabinet: {}", e);
            DomainError::InternalError
        })?;
        Ok(())
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

    async fn delete_unused_pending_destruction(&self) -> Result<(), DomainError> {
        let result = Entity::delete_many()
            .filter(Column::Used.eq(false))
            .filter(Column::PendingDestruction.eq(true))
            .exec(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to delete unused pending destruction cabinet: {}", e);
                DomainError::InternalError
            })?;
        log::info!(
            "Deleted {} unused pending destruction cabinets.",
            result.rows_affected
        );
        Ok(())
    }

    async fn update_by_code(&self, cabinet: Cabinet) -> Result<(), DomainError> {
        use sea_orm::ActiveValue;
        let model = Model::from(cabinet);
        let mut active_model = ActiveModel::new();
        active_model.code = ActiveValue::Unchanged(model.code);
        active_model.name = ActiveValue::Set(model.name);
        active_model.description = ActiveValue::Set(model.description);
        active_model.password = ActiveValue::Set(model.password);
        active_model.used = ActiveValue::Set(model.used);
        active_model.pending_destruction = ActiveValue::Set(model.pending_destruction);
        active_model.update_at = ActiveValue::Set(model.update_at);
        active_model.version = ActiveValue::Set(model.version + 1);
        active_model.update(&self.connection).await.map_err(|e| {
            log::error!("Failed to update cabinet: {}", e);
            DomainError::InternalError
        })?;
        Ok(())
    }

    async fn update_pending_destruction_by_codes(
        &self,
        codes: Vec<i64>,
        pending_destruction: bool,
    ) -> Result<(), DomainError> {
        use sea_orm::sea_query;
        Entity::update_many()
            .col_expr(
                Column::PendingDestruction,
                sea_query::Expr::value(pending_destruction),
            )
            .col_expr(
                Column::UpdateAt,
                sea_query::Expr::value(Local::now().naive_local()),
            )
            .col_expr(
                Column::Version,
                sea_query::Expr::col(Column::Version).add(1),
            )
            .filter(Column::Code.is_in(codes))
            .exec(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to mark pending destruction: {}", e);
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

    async fn count_used(&self) -> Result<u64, DomainError> {
        Entity::find()
            .filter(Column::Used.eq(true))
            .count(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to count used cabinet: {}", e);
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

    async fn max_code(&self) -> Result<Option<i64>, DomainError> {
        use sea_orm::{QueryOrder, QuerySelect};
        Entity::find()
            .filter(Column::PendingDestruction.eq(false))
            .order_by_desc(Column::Code)
            .limit(1)
            .one(&self.connection)
            .await
            .map(|model| model.map(|model| model.code))
            .map_err(|e| {
                log::error!("Failed to find max code: {}", e);
                DomainError::InternalError
            })
    }

    async fn find_by_code(&self, code: i64) -> Result<Option<Cabinet>, DomainError> {
        Entity::find_by_id(code)
            .one(&self.connection)
            .await
            .map(|model| model.map(|model| model.into()))
            .map_err(|e| {
                log::error!("Failed to find cabinet: {}", e);
                DomainError::InternalError
            })
    }

    async fn list_unused(&self) -> Result<Vec<Cabinet>, DomainError> {
        Entity::find()
            .filter(Column::Used.eq(false))
            .filter(Column::PendingDestruction.eq(false))
            .all(&self.connection)
            .await
            .map(|models| models.into_iter().map(|model| model.into()).collect())
            .map_err(|e| {
                log::error!("Failed to list unused cabinet: {}", e);
                DomainError::InternalError
            })
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
            used: value.used,
            pending_destruction: value.pending_destruction,
            create_at: value.create_at.map(|e| e.naive_local()).unwrap_or(now),
            update_at: value.update_at.map(|e| e.naive_local()).unwrap_or(now),
            version: value.version.unwrap_or(1),
        }
    }
}

impl From<Model> for Cabinet {
    fn from(value: Model) -> Self {
        Cabinet {
            code: value.code,
            name: value.name,
            description: value.description,
            password: value.password,
            used: value.used,
            pending_destruction: value.pending_destruction,
            create_at: Local.from_local_datetime(&value.create_at).single(),
            update_at: Local.from_local_datetime(&value.update_at).single(),
            version: Some(value.version),
        }
    }
}
