use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Error)]
pub enum KeyError {
    #[error("Invalid key size")]
    InvalidKeySize,
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Key {
    bytes: [u8; 32],
}

impl Key {
    pub fn new(bytes: Vec<u8>) -> Result<Key, String> {
        let bytes: [u8; 32] = match bytes.try_into() {
            Ok(bytes) => bytes,
            Err(err) => return Err(format!("{}: {:?}", KeyError::InvalidKeySize, err)),
        };

        Ok(Key { bytes })
    }

    pub fn to_hex(&self) -> String {
        let bytes: &[u8] = &self.bytes;
        let string = hex::encode(&bytes);
        bytes.to_owned().zeroize();
        string
    }
}

pub struct Keypair {
    pub private: Key,
    pub public: Key,
}
