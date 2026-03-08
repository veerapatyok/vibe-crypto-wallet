use crate::domain::crypto::AirgapProvider;
use anyhow::Result;
use ur::Encoder;

pub struct UrAdapter;

impl AirgapProvider for UrAdapter {
    fn encode_to_ur(&self, data: &[u8]) -> Result<Vec<String>> {
        let mut encoder = Encoder::bytes(data, 200).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let mut fragments = Vec::new();
        for _ in 0..encoder.fragment_count() {
            let fragment = encoder
                .next_part()
                .map_err(|e| anyhow::anyhow!("{:?}", e))?;
            fragments.push(fragment);
        }
        Ok(fragments)
    }
}
