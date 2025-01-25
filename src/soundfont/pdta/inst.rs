use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone)]
pub struct Instruments {
    pub contents: Vec<Instrument>,
}

impl Default for Instruments {
    fn default() -> Self {
        Self::new()
    }
}

impl Instruments {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }

    pub fn to_riff(&self) -> ChunkContents {
        let mut contents = vec![];
        for gen in &self.contents {
            contents.append(&mut gen.to_bytes());
        }
        ChunkContents::Data(ChunkId { value: *b"inst" }, contents)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Instrument {
    pub name: String,
    pub inst_bag_idx: u16,
}

impl Instrument {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        let mut name_bytes = self.name.as_bytes().to_vec();
        assert!(name_bytes.len() <= 20);
        name_bytes.resize(20, 0);

        bytes.append(&mut name_bytes);
        bytes.append(&mut self.inst_bag_idx.to_le_bytes().to_vec());

        assert_eq!(bytes.len(), 22);
        bytes
    }
}
