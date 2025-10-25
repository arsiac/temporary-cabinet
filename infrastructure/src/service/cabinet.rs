use crate::repository::cabinet::{CabinetItemRepository, CabinetRepository};
use domain::service::cabinet::CabinetService;

/// Create cabinet service
pub fn create_cabinet_service(
    connection: sea_orm::DatabaseConnection,
    data_folder: &std::path::Path,
    cabinets_number: u64,
) -> CabinetService<CabinetRepository, CabinetItemRepository> {
    CabinetService::new(
        CabinetRepository::new(connection.clone()),
        CabinetItemRepository::new(connection, data_folder.join("files")),
        cabinets_number,
    )
}
