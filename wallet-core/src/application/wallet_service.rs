use crate::domain::{AirgapProvider, Chain, CryptoProvider, MnemonicProvider, Wallet};
use std::sync::Arc;

pub struct WalletService {
    mnemonic_provider: Arc<dyn MnemonicProvider + Send + Sync>,
    crypto_provider: Arc<dyn CryptoProvider + Send + Sync>,
    airgap_provider: Arc<dyn AirgapProvider + Send + Sync>,
}

impl WalletService {
    pub fn new(
        mnemonic_provider: Arc<dyn MnemonicProvider + Send + Sync>,
        crypto_provider: Arc<dyn CryptoProvider + Send + Sync>,
        airgap_provider: Arc<dyn AirgapProvider + Send + Sync>,
    ) -> Self {
        Self {
            mnemonic_provider,
            crypto_provider,
            airgap_provider,
        }
    }

    pub fn create_random_wallet(&self, word_count: u8) -> Result<Wallet, String> {
        let phrase = self.mnemonic_provider.generate_mnemonic(word_count)?;
        Ok(Wallet::new(phrase))
    }

    pub fn import_wallet(&self, phrase: &str) -> Result<Wallet, String> {
        self.mnemonic_provider.validate_mnemonic(phrase)?;
        Ok(Wallet::new(phrase.to_string()))
    }

    pub fn derive_address(
        &self,
        wallet: &Wallet,
        chain: Chain,
        pin: Option<&str>,
    ) -> Result<String, String> {
        let seed = self.mnemonic_provider.get_seed(&wallet.mnemonic, pin)?;
        self.crypto_provider.derive_address(&seed, chain)
    }

    pub fn sign_evm_hash(
        &self,
        wallet: &Wallet,
        message_hash: [u8; 32],
        pin: Option<&str>,
    ) -> Result<Vec<u8>, String> {
        let seed = self.mnemonic_provider.get_seed(&wallet.mnemonic, pin)?;
        self.crypto_provider.sign_evm_hash(&seed, message_hash)
    }

    pub fn encode_to_ur(&self, data: &[u8]) -> Result<Vec<String>, String> {
        self.airgap_provider.encode_to_ur(data)
    }
}
