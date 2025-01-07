//! Level 0 RIFF chunk 2/3 - sdta-list chunk
//! Contains sample binary data

use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone)]
pub struct SdtaList {
    /// 16-bit
    pub smpl: Vec<i16>,
    // TODO: /// 24-bit extension
    // pub sm24: Vec<u8>
}

impl From<SdtaList> for ChunkContents {
    fn from(value: SdtaList) -> Self {
        let mut smpl = vec![];
        for sample in value.smpl {
            smpl.append(&mut sample.to_le_bytes().to_vec());
        }
        let contents = vec![ChunkContents::Data(ChunkId { value: *b"smpl" }, smpl)];

        ChunkContents::Children(riff::LIST_ID, ChunkId { value: *b"sdta" }, contents)
    }
}

impl SdtaList {
    pub fn new() -> Self {
        Self { smpl: vec![] }
    }
}
