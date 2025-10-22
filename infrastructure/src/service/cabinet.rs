use crate::repository::cabinet::{CabinetItemRepository, CabinetRepository};
use domain::service::cabinet::CabinetService;

/// Create cabinet service
pub fn create_cabinet_service(
    connection: &sea_orm::DatabaseConnection,
    data_folder: &std::path::Path,
) -> CabinetService<CabinetRepository, CabinetItemRepository> {
    CabinetService::new(
        CabinetRepository::new(connection.clone()),
        CabinetItemRepository::new(connection.clone(), data_folder.join("files")),
    )
}
