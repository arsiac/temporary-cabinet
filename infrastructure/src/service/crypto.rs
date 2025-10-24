use crate::repository::crypto::CryptoKeypairRepository;
use domain::service::crypto::Sm2CryptoService;

pub fn create_sm2_crypto_service(
    connection: sea_orm::DatabaseConnection,
) -> Sm2CryptoService<CryptoKeypairRepository> {
    Sm2CryptoService::new(CryptoKeypairRepository::new(connection))
}
