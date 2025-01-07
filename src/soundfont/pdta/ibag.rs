use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone)]
pub struct InstBag {
    pub contents: Vec<InstZone>,
}

impl From<InstBag> for ChunkContents {
    fn from(value: InstBag) -> Self {
        println!("Packing ibag: {value:?}");

        let mut contents = vec![];
        for bag in value.contents {
            contents.append(&mut bag.as_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"ibag" }, contents)
    }
}

impl InstBag {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct InstZone {
    pub gen_idx: u16,
    pub mod_idx: u16,
}

impl InstZone {
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
