use crate::domain::MnemonicProvider;
use bip39::{Language, Mnemonic, MnemonicType, Seed};

pub struct Bip39Adapter;

impl MnemonicProvider for Bip39Adapter {
    fn generate_mnemonic(&self, word_count: u8) -> Result<String, String> {
        let m_type = match word_count {
            12 => MnemonicType::Words12,
            15 => MnemonicType::Words15,
            18 => MnemonicType::Words18,
            21 => MnemonicType::Words21,
            24 => MnemonicType::Words24,
            _ => return Err("Invalid word count".to_string()),
        };
        Ok(Mnemonic::new(m_type, Language::English)
            .phrase()
            .to_string())
    }

    fn validate_mnemonic(&self, phrase: &str) -> Result<(), String> {
        let _ = Mnemonic::from_phrase(phrase, Language::English).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn get_seed(&self, phrase: &str, passphrase: Option<&str>) -> Result<Vec<u8>, String> {
        let mnemonic =
            Mnemonic::from_phrase(phrase, Language::English).map_err(|e| e.to_string())?;
        let seed = Seed::new(&mnemonic, passphrase.unwrap_or(""));
        Ok(seed.as_bytes().to_vec())
    }
}
