pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::WalletService;
pub use domain::{Chain, Wallet};
pub use infrastructure::{Bip39Adapter, CryptoAdapter};

pub mod airgap {
    use ur::Encoder;

    pub fn encode_to_ur(data: &[u8], _type_str: &str) -> Result<Vec<String>, String> {
        let mut encoder = Encoder::bytes(data, 200).map_err(|e| e.to_string())?;
        let mut fragments = Vec::new();
        for _ in 0..encoder.fragment_count() {
            let fragment = encoder.next_part().map_err(|e| e.to_string())?;
            fragments.push(fragment);
        }
        Ok(fragments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_hexagonal_wallet() {
        let bip39 = Arc::new(Bip39Adapter);
        let crypto = Arc::new(CryptoAdapter);
        let service = WalletService::new(bip39, crypto);

        let wallet = service.create_random_wallet(24).unwrap();
        assert_eq!(wallet.mnemonic.split_whitespace().count(), 24);
        let evm_address = service.derive_address(&wallet, Chain::Evm, None).unwrap();
        let sol_address = service
            .derive_address(&wallet, Chain::Solana, None)
            .unwrap();

        assert!(evm_address.starts_with("0x"));
        assert!(sol_address.len() >= 32);
    }

    #[test]
    fn test_pin_derivation() {
        let bip39 = Arc::new(Bip39Adapter);
        let crypto = Arc::new(CryptoAdapter);
        let service = WalletService::new(bip39, crypto);

        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let wallet = service.import_wallet(phrase).unwrap();

        // No PIN
        let addr_no_pin = service.derive_address(&wallet, Chain::Evm, None).unwrap();
        // PIN 1234
        let addr_pin_1 = service
            .derive_address(&wallet, Chain::Evm, Some("1234"))
            .unwrap();
        // PIN 5678
        let addr_pin_2 = service
            .derive_address(&wallet, Chain::Evm, Some("5678"))
            .unwrap();

        assert_ne!(addr_no_pin, addr_pin_1);
        assert_ne!(addr_pin_1, addr_pin_2);
        assert_eq!(addr_no_pin, "0xa3611257ea0f2360811a9c44fa6d908939165252");
    }
}
