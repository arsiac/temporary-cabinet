use crate::entity::cabinet::{ActiveModel, Column, Entity, Model};
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

    async fn update_by_code(&self, cabinet: Cabinet) -> Result<(), DomainError> {
        let code = cabinet.code;
        let model = Model::from(cabinet);
        let active_model = ActiveModel::from(model);
        Entity::update_many()
            .set(active_model)
            .filter(Column::Code.eq(code))
            .exec(&self.connection)
            .await
            .map_err(|e| {
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
            create_at: now,
            update_at: now,
            version: 1,
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
        }
    }
}
