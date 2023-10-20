use crate::types::keys::{Key, Keypair};
use bip32::XPrv;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HDWalletError {
    #[error("Unable to parse path")]
    PathError,
    #[error("Unable to derive keys from path")]
    DerivationError,
    #[error("Could not create secret key from bytes")]
    SecretKeyError,
    #[error("Invalid seed length")]
    InvalidSeed,
}

pub struct HDWallet {
    seed: [u8; 64],
}

impl HDWallet {
    pub fn new(seed: &[u8]) -> Result<HDWallet, String> {
        let seed: [u8; 64] = match seed.clone().try_into() {
            Ok(seed) => seed,
            Err(err) => return Err(format!("{}: {:?}", HDWalletError::InvalidSeed, err)),
        };

        Ok(HDWallet { seed })
    }

    /// Derive account from a seed and a path
    pub fn derive(&self, path: String) -> Result<Keypair, String> {
        // Path should be a parse-able string
        let path = path
            .parse()
            .map_err(|err| format!("{}: {:?}", HDWalletError::PathError, err))?;
        // BIP32 Extended Private Key
        let xprv = XPrv::derive_from_path(&self.seed, &path)
            .map_err(|_| HDWalletError::DerivationError.to_string())?;

        let prv_bytes: &mut [u8] = &mut xprv.private_key().to_bytes();

        // Create ed25519 keypair
        let secret_key = ed25519_dalek::SecretKey::from_bytes(prv_bytes)
            .map_err(|_| HDWalletError::SecretKeyError.to_string())?;
        let public_key = ed25519_dalek::PublicKey::from(&secret_key);

        let private = Key::new(Vec::from(secret_key.to_bytes()))?;
        let public = Key::new(Vec::from(public_key.to_bytes()))?;

        Ok(Keypair { private, public })
    }
}
