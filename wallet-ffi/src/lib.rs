use std::sync::Arc;
use wallet_core::{
    Bip39Adapter, Chain as DomainChain, CryptoAdapter, UrAdapter, Wallet as DomainWallet,
    WalletService,
};

uniffi::setup_scaffolding!();

#[derive(uniffi::Enum)]
pub enum Chain {
    Evm,
    Solana,
}

impl From<Chain> for DomainChain {
    fn from(chain: Chain) -> Self {
        match chain {
            Chain::Evm => DomainChain::Evm,
            Chain::Solana => DomainChain::Solana,
        }
    }
}

#[derive(uniffi::Object)]
pub struct Wallet {
    inner: DomainWallet,
    service: Arc<WalletService>,
}

#[uniffi::export]
impl Wallet {
    #[uniffi::constructor]
    pub fn new_random(word_count: u8) -> Result<Self, String> {
        let bip39 = Arc::new(Bip39Adapter);
        let crypto = Arc::new(CryptoAdapter);
        let airgap = Arc::new(UrAdapter);
        let service = Arc::new(WalletService::new(bip39, crypto, airgap));
        let inner = service.create_random_wallet(word_count)?;
        Ok(Self {
            inner,
            service: service.clone(),
        })
    }

    #[uniffi::constructor]
    pub fn from_mnemonic(phrase: String) -> Result<Self, String> {
        let bip39 = Arc::new(Bip39Adapter);
        let crypto = Arc::new(CryptoAdapter);
        let airgap = Arc::new(UrAdapter);
        let service = Arc::new(WalletService::new(bip39, crypto, airgap));
        let inner = service.import_wallet(&phrase)?;
        Ok(Self { inner, service })
    }

    pub fn get_mnemonic(&self) -> String {
        self.inner.mnemonic.clone()
    }

    pub fn derive_address(&self, chain: Chain, pin: Option<String>) -> Result<String, String> {
        self.service
            .derive_address(&self.inner, chain.into(), pin.as_deref())
    }

    pub fn sign_evm_hash(
        &self,
        message_hash: Vec<u8>,
        pin: Option<String>,
    ) -> Result<Vec<u8>, String> {
        if message_hash.len() != 32 {
            return Err("Message hash must be 32 bytes".to_string());
        }
        let mut hash_arr = [0u8; 32];
        hash_arr.copy_from_slice(&message_hash);
        self.service
            .sign_evm_hash(&self.inner, hash_arr, pin.as_deref())
    }

    pub fn encode_qr_fragments(
        &self,
        data: Vec<u8>,
        _type_str: String,
    ) -> Result<Vec<String>, String> {
        self.service.encode_to_ur(&data)
    }
}
