use super::chain::Chain;

pub trait MnemonicProvider {
    fn generate_mnemonic(&self, word_count: u8) -> Result<String, String>;
    fn validate_mnemonic(&self, phrase: &str) -> Result<(), String>;
    fn get_seed(&self, phrase: &str, passphrase: Option<&str>) -> Result<Vec<u8>, String>;
}

pub trait CryptoProvider {
    fn derive_address(&self, seed: &[u8], chain: Chain) -> Result<String, String>;
    fn sign_evm_hash(&self, seed: &[u8], message_hash: [u8; 32]) -> Result<Vec<u8>, String>;
}
