#[derive(Clone, Debug)]
pub struct Wallet {
    pub mnemonic: String,
}

impl Wallet {
    pub fn new(mnemonic: String) -> Self {
        Self { mnemonic }
    }
}
