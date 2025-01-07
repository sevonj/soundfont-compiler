pub mod info;
pub mod pdta;
pub mod sdta;

use riff::{ChunkContents, ChunkId};

use info::InfoList;
use pdta::PdtaList;
use sdta::SdtaList;

pub struct SoundFont2 {
    pub info: InfoList,
    pub sdta: SdtaList,
    pub pdta: PdtaList,
}

impl SoundFont2 {
    pub fn new() -> Self {
        Self {
            info: InfoList::default(),
            sdta: SdtaList::new(),
            pdta: PdtaList::default(),
        }
    }
    pub fn create_riff(&self) -> ChunkContents {
        let contents = vec![
            self.info.clone().into(),
            self.sdta.clone().into(),
            self.pdta.clone().into(),
        ];

        ChunkContents::Children(riff::RIFF_ID, ChunkId { value: *b"sfbk" }, contents)
    }
}

/// Convert string into sf2-compliant bytes.
pub fn sf_string_to_bytes(value: &String) -> Vec<u8> {
    let mut bytes = value.as_bytes().to_vec();
    if bytes.len() % 2 != 0 {
        bytes.push(0);
    }
    // TODO: Error placeholder
    assert!(bytes.len() <= 256);
    bytes
}

//
// misplaced stuff below
//

enum SFSampleLink {
    MonoSample = 1,
    RightSample = 2,
    LeftSample = 4,
    LinkedSample = 8,
    RomMonoSample = 0x8001,
    RomRightSample = 0x8002,
    RomLeftSample = 0x8004,
    RomLinkedSample = 0x8008,
}
