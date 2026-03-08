use crate::domain::{Chain, CryptoProvider};
use anyhow::Result;
use ed25519_dalek::SigningKey as EdSigningKey;
use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use sha3::{Digest, Keccak256};

pub struct CryptoAdapter;

impl CryptoProvider for CryptoAdapter {
    fn derive_address(&self, seed: &[u8], chain: Chain) -> Result<String> {
        match chain {
            Chain::Evm => self.derive_evm_address(seed),
            Chain::Solana => self.derive_solana_address(seed),
        }
    }

    fn sign_evm_hash(&self, seed: &[u8], message_hash: [u8; 32]) -> Result<Vec<u8>> {
        let signing_key = SigningKey::from_slice(&seed[0..32])?;
        let signature: Signature = signing_key.sign(&message_hash);
        Ok(signature.to_bytes().to_vec())
    }
}

impl CryptoAdapter {
    fn derive_evm_address(&self, seed: &[u8]) -> Result<String> {
        let signing_key = SigningKey::from_slice(&seed[0..32])?;
        let verifying_key = VerifyingKey::from(&signing_key);
        let public_key_point = verifying_key.to_encoded_point(false);
        let public_key_bytes = public_key_point.as_bytes();

        let mut hasher = Keccak256::new();
        hasher.update(&public_key_bytes[1..]);
        let hash = hasher.finalize();

        let address = &hash[12..];
        Ok(format!("0x{}", hex::encode(address)))
    }

    fn derive_solana_address(&self, seed: &[u8]) -> Result<String> {
        let seed_bytes: [u8; 32] = seed[0..32].try_into()?;
        let signing_key = EdSigningKey::from_bytes(&seed_bytes);
        let public_key = signing_key.verifying_key();
        let address = bs58::encode(public_key.as_bytes()).into_string();
        Ok(address)
    }
}
