//! Level 0 RIFF chunk 2/3 - sdta-list chunk
//! Contains sample binary data

use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone, Default)]
pub struct SdtaList {
    /// 16-bit
    pub smpl: Vec<i16>,
    /// 24-bit extension
    pub sm24: Option<Vec<u8>>,
}

impl SdtaList {
    pub fn to_riff(&self) -> ChunkContents {
        let mut smpl_bytes = vec![];
        for sample in &self.smpl {
            smpl_bytes.append(&mut sample.to_le_bytes().to_vec());
        }
        let mut contents = vec![ChunkContents::Data(ChunkId { value: *b"smpl" }, smpl_bytes)];
        if let Some(sm24) = &self.sm24 {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"sm24" },
                sm24.clone(),
            ));
        }

        ChunkContents::Children(riff::LIST_ID, ChunkId { value: *b"sdta" }, contents)
    }
}

impl SdtaList {
    pub fn new(smpl: Vec<i16>, sm24: Option<Vec<u8>>) -> Self {
        Self { smpl, sm24 }
    }
}
