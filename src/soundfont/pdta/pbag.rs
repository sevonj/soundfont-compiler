use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone)]
pub struct PresetBag {
    pub contents: Vec<PresetZone>,
}

impl From<PresetBag> for ChunkContents {
    fn from(value: PresetBag) -> Self {
        println!("Packing pbag: {value:?}");

        let mut contents = vec![];
        for bag in value.contents {
            contents.append(&mut bag.as_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"pbag" }, contents)
    }
}

impl PresetBag {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct PresetZone {
    pub gen_idx: u16,
    pub mod_idx: u16,
}

impl PresetZone {
    pub fn new(gen_idx: u16, mod_idx: u16) -> Self {
        Self { gen_idx, mod_idx }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.gen_idx.to_le_bytes().to_vec());
        bytes.append(&mut self.mod_idx.to_le_bytes().to_vec());
        bytes
    }

    /// Null valued terminal record
    pub fn terminal() -> Self {
        Self {
            gen_idx: 0,
            mod_idx: 0,
        }
    }
}
