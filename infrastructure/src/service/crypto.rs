use crate::repository::crypto::CryptoKeypairRepository;
use domain::service::crypto::Sm2CryptoService;

/// Create SM2 crypto service
pub fn create_sm2_crypto_service(
    connection: sea_orm::DatabaseConnection,
    max_keypair_number: u64,
) -> Sm2CryptoService<CryptoKeypairRepository> {
    Sm2CryptoService::new(CryptoKeypairRepository::new(connection), max_keypair_number)
}
