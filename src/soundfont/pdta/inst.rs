use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone)]
pub struct Instruments {
    pub contents: Vec<Instrument>,
}

impl From<Instruments> for ChunkContents {
    fn from(value: Instruments) -> Self {
        println!("Packing inst: {value:?}");

        let mut contents = vec![];
        for gen in value.contents {
            contents.append(&mut gen.as_bytes());
        }
        ChunkContents::Data(ChunkId { value: *b"inst" }, contents)
    }
}

impl Instruments {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct Instrument {
    pub name: String,
    pub inst_bag_idx: u16,
}

impl Default for Instrument {
    fn default() -> Self {
        Self {
            name: Default::default(),
            inst_bag_idx: Default::default(),
        }
    }
}

impl Instrument {
    pub fn as_bytes(&self) -> Vec<u8> {
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
