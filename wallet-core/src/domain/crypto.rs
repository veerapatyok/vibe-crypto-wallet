use super::chain::Chain;
use anyhow::Result;

pub trait MnemonicProvider {
    fn generate_mnemonic(&self, word_count: u8) -> Result<String>;
    fn validate_mnemonic(&self, phrase: &str) -> Result<()>;
    fn get_seed(&self, phrase: &str, passphrase: Option<&str>) -> Result<Vec<u8>>;
}

pub trait CryptoProvider {
    fn derive_address(&self, seed: &[u8], chain: Chain) -> Result<String>;
    fn sign_evm_hash(&self, seed: &[u8], message_hash: [u8; 32]) -> Result<Vec<u8>>;
}

pub trait AirgapProvider {
    fn encode_to_ur(&self, data: &[u8]) -> Result<Vec<String>>;
}
