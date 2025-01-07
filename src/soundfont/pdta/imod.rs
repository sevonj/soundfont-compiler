use riff::{ChunkContents, ChunkId};

/// Modulator list for Presets and Instruments
#[derive(Debug, Clone)]
pub struct InstModList {
    pub contents: Vec<InstrumentMod>,
}

impl From<InstModList> for ChunkContents {
    fn from(value: InstModList) -> Self {
        println!("Packing imod: {value:?}");

        let mut contents = vec![];
        //for gen in value.contents {
        //    contents.append(&mut gen.as_bytes());
        //}

        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"pmod" }, contents)
    }
}

impl InstModList {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct InstrumentMod {}
