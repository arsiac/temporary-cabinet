use domain::error::DomainError;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

/// Begin a database transaction
pub async fn begin_transaction(
    connection: &DatabaseConnection,
) -> Result<DatabaseTransactionWrapper, DomainError> {
    let transaction = connection.begin().await;
    if let Err(e) = transaction {
        log::error!("Failed to begin transaction: {}", e);
        return Err(DomainError::InternalError);
    }
    Ok(DatabaseTransactionWrapper::new(transaction.unwrap()))
}

pub struct DatabaseTransactionWrapper {
    transaction: DatabaseTransaction,
}

impl DatabaseTransactionWrapper {
    pub fn new(transaction: DatabaseTransaction) -> Self {
        Self { transaction }
    }
}

impl DatabaseTransactionWrapper {
    /// Rollback a database transaction
    pub async fn rollback(self) -> Result<(), DomainError> {
        if let Err(e) = self.transaction.rollback().await {
            log::error!("Failed to rollback transaction: {}", e);
            return Err(DomainError::InternalError);
        }
        Ok(())
    }

    /// Commit a database transaction
    pub async fn commit(self) -> Result<(), DomainError> {
        if let Err(e) = self.transaction.commit().await {
            log::error!("Failed to commit transaction: {}", e);
            return Err(DomainError::InternalError);
        }
        Ok(())
    }
}
