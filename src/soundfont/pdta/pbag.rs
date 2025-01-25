use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone)]
pub struct PresetBag {
    pub contents: Vec<PresetZone>,
}

impl Default for PresetBag {
    fn default() -> Self {
        Self::new()
    }
}

impl PresetBag {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }

    pub fn to_riff(&self) -> ChunkContents {
        let mut contents = vec![];
        for bag in &self.contents {
            contents.append(&mut bag.to_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"pbag" }, contents)
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

    pub fn to_bytes(&self) -> Vec<u8> {
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
