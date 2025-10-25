use crate::entity::keypair::{ActiveModel, Column, Entity, Model};
use chrono::{DateTime, Local, TimeZone};
use domain::entity::crypto::CryptoKeypair;
use domain::error::DomainError;
use domain::repository::crypto::CryptoKeypairRepository as Repository;
use sea_orm::prelude::*;

pub struct CryptoKeypairRepository {
    connection: DatabaseConnection,
}

impl CryptoKeypairRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl Repository for CryptoKeypairRepository {
    async fn save(&self, keypair: CryptoKeypair) -> Result<CryptoKeypair, DomainError> {
        let model: Model = keypair.into();
        let active_model: ActiveModel = model.into();
        let res = active_model.insert(&self.connection).await.map_err(|e| {
            log::error!("Failed to save keypair: {e}");
            DomainError::InternalError
        })?;
        Ok(res.into())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<(), DomainError> {
        Entity::delete_by_id(id)
            .exec(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to delete keypair by id {id}: {e}");
                DomainError::InternalError
            })?;
        Ok(())
    }

    async fn delete_expired(&self, time: DateTime<Local>) -> Result<u64, DomainError> {
        let result = Entity::delete_many()
            .filter(Column::ExpireAt.lte(time.naive_local()))
            .exec(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to delete expired keypairs: {e}");
                DomainError::InternalError
            })?;
        Ok(result.rows_affected)
    }

    async fn find_by_public_key(
        &self,
        public_key: &str,
    ) -> Result<Option<CryptoKeypair>, DomainError> {
        let result = Entity::find()
            .filter(Column::PublicKey.eq(public_key))
            .one(&self.connection)
            .await
            .map_err(|e| {
                log::error!("Failed to find keypair by public key {public_key}: {e}");
                DomainError::InternalError
            })?;
        Ok(result.map(|model| model.into()))
    }
}

impl From<CryptoKeypair> for Model {
    fn from(value: CryptoKeypair) -> Self {
        let now = chrono::Local::now().naive_local();
        Self {
            id: value.id.unwrap_or_else(Uuid::now_v7),
            secret_key: value.secret_key,
            public_key: value.public_key,
            expire_at: value.expire_at.naive_local(),
            create_at: value.create_at.map(|e| e.naive_local()).unwrap_or(now),
            update_at: value.update_at.map(|e| e.naive_local()).unwrap_or(now),
            version: value.version.unwrap_or(1),
        }
    }
}

impl From<Model> for CryptoKeypair {
    fn from(value: Model) -> Self {
        Self {
            id: Some(value.id),
            secret_key: value.secret_key,
            public_key: value.public_key,
            expire_at: Local
                .from_local_datetime(&value.expire_at)
                .single()
                .unwrap(),
            create_at: Local.from_local_datetime(&value.create_at).single(),
            update_at: Local.from_local_datetime(&value.update_at).single(),
            version: Some(value.version),
        }
    }
}
