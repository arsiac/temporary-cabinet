use chrono::Local;

use crate::entity::crypto::CryptoKeypair;
use crate::error::DomainError;
use crate::error::crypto::CryptoError;
use crate::repository::crypto::CryptoKeypairRepository;

pub struct Sm2CryptoService<R: CryptoKeypairRepository> {
    crypto_keypair_repository: R,
    max_keypair_number: u64,
}

impl<R: CryptoKeypairRepository> Sm2CryptoService<R> {
    pub fn new(crypto_keypair_repository: R, max_keypair_number: u64) -> Self {
        Sm2CryptoService {
            crypto_keypair_repository,
            max_keypair_number,
        }
    }
}

impl<R: CryptoKeypairRepository> Sm2CryptoService<R> {
    /// Generate a new ECIES keypair.
    pub async fn generate_keypair(&self) -> Result<CryptoKeypair, DomainError> {
        use chrono::{Duration, Local};

        let count = self.crypto_keypair_repository.count().await?;
        if count >= self.max_keypair_number {
            log::warn!(
                "Maximum number({}) of keypairs reached.",
                self.max_keypair_number
            );
            return Err(CryptoError::MaxKeypairCountReached)?;
        }

        let (pk, sk) = gm_sm2::key::gen_keypair().map_err(|e| {
            log::error!("Generate SM2 keypair failed: {e}");
            CryptoError::KeypairGenerationFailed
        })?;
        let keypair = CryptoKeypair::new(
            sk2hex(&sk),
            pk2hex(&pk),
            Local::now() + Duration::minutes(5),
        );
        log::debug!(
            "Generated new SM2 keypair with public key '{}'",
            &keypair.public_key
        );
        self.crypto_keypair_repository.save(keypair).await
    }

    /// Delete keypair by id
    pub async fn delete_by_id(&self, id: uuid::Uuid) -> Result<(), DomainError> {
        log::debug!("Deleting keypair with id '{id}'");
        self.crypto_keypair_repository.delete_by_id(id).await
    }

    /// Delete expired keypairs
    pub async fn delete_expired(&self) -> Result<u64, DomainError> {
        log::debug!("Deleting expired keypairs");
        self.crypto_keypair_repository
            .delete_expired(Local::now())
            .await
    }

    /// Get a keypair by its public key.
    pub async fn get_by_public_key(
        &self,
        public_key: &str,
    ) -> Result<Option<CryptoKeypair>, DomainError> {
        self.crypto_keypair_repository
            .find_by_public_key(public_key)
            .await
    }

    /// Get a effective keypair by its public key.
    pub async fn get_effective_by_public_key(
        &self,
        public_key: &str,
    ) -> Result<CryptoKeypair, DomainError> {
        let keypair = self
            .crypto_keypair_repository
            .find_by_public_key(public_key)
            .await?;
        if keypair.is_none() {
            return Err(CryptoError::NotFound)?;
        }
        let keypair = keypair.unwrap();
        if keypair.expire_at < chrono::Local::now() {
            log::error!(
                "Keypair with public key '{}' has expired at {}",
                public_key,
                keypair.expire_at
            );
            return Err(CryptoError::KeypairExpired)?;
        }
        Ok(keypair)
    }
}

pub fn sk2hex(sk: &gm_sm2::key::Sm2PrivateKey) -> String {
    sk.to_hex_string()
}

pub fn pk2hex(pk: &gm_sm2::key::Sm2PublicKey) -> String {
    pk.to_hex_string(false)
}

pub fn hex2sk(hex_str: &str) -> Result<gm_sm2::key::Sm2PrivateKey, CryptoError> {
    gm_sm2::key::Sm2PrivateKey::from_hex_string(hex_str).map_err(|e| {
        log::error!("Invalid SM2 secret key '{hex_str}': {e:?}");
        CryptoError::InvalidSecretKey
    })
}

pub fn hex2pk(hex_str: &str) -> Result<gm_sm2::key::Sm2PublicKey, CryptoError> {
    gm_sm2::key::Sm2PublicKey::from_hex_string(hex_str).map_err(|e| {
        log::error!("Invalid SM2 public key '{hex_str}': {e:?}");
        CryptoError::InvalidPublicKey
    })
}

pub fn hex2bytes(hex_str: &str) -> Result<Vec<u8>, CryptoError> {
    hex::decode(hex_str).map_err(|e| {
        log::error!("Invalid hex string '{hex_str}': {e:?}");
        CryptoError::InvalidHexString
    })
}

pub fn decrypt(
    sk: &gm_sm2::key::Sm2PrivateKey,
    encrypted_data: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    sk.decrypt(encrypted_data, false, gm_sm2::key::Sm2Model::C1C3C2)
        .map_err(|e| {
            log::error!("Failed to decrypt data: {e:?}");
            CryptoError::DecryptionFailed
        })
}

pub fn decrypt_hex_to_plaintext(
    sk: &gm_sm2::key::Sm2PrivateKey,
    encrypted_hex: &str,
) -> Result<String, CryptoError> {
    let encrypted_data = hex2bytes(encrypted_hex)?;
    let bytes = decrypt(sk, &encrypted_data)?;
    String::from_utf8(bytes).map_err(|e| {
        log::error!("Failed to convert decrypted bytes to utf-8 string: {e}");
        CryptoError::DecryptionFailed
    })
}
