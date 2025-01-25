mod error;
pub mod info;
pub mod pdta;
pub mod sdta;

use riff::{ChunkContents, ChunkId};

pub use error::SoundfontError;
use info::InfoList;
use pdta::PdtaList;
use sdta::SdtaList;

#[derive(Debug, Default)]
pub struct SoundFont2 {
    pub info: InfoList,
    pub sdta: SdtaList,
    pub pdta: PdtaList,
}

impl SoundFont2 {
    pub fn new(info: InfoList, sdta: SdtaList, pdta: PdtaList) -> Self {
        Self { info, sdta, pdta }
    }

    pub fn to_riff(&self) -> ChunkContents {
        let contents = vec![
            self.info.to_riff(),
            self.sdta.to_riff(),
            self.pdta.to_riff(),
        ];

        ChunkContents::Children(riff::RIFF_ID, ChunkId { value: *b"sfbk" }, contents)
    }
}

/// Convert to bytes, with SF2-compliant terminators.
fn string_to_bytes(value: &String) -> Vec<u8> {
    let mut bytes = value.as_bytes().to_vec();
    if bytes.len() % 2 != 0 {
        bytes.push(0);
    }
    bytes
}

//
// misplaced stuff below
//

#[allow(dead_code)]
enum SFSampleLink {
    Mono = 1,
    Right = 2,
    Left = 4,
    Linked = 8,
    RomMono = 0x8001,
    RomRight = 0x8002,
    RomLeft = 0x8004,
    RomLinked = 0x8008,
}
