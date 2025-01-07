use riff::{ChunkContents, ChunkId};

/// Modulator list for Presets and Instruments
#[derive(Debug, Clone)]
pub struct PresetModList {
    pub contents: Vec<PresetMod>,
}

impl From<PresetModList> for ChunkContents {
    fn from(value: PresetModList) -> Self {
        println!("Packing pmod: {value:?}");

        let mut contents = vec![];
        //for gen in value.contents {
        //    contents.append(&mut gen.as_bytes());
        //}

        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"pmod" }, contents)
    }
}

impl PresetModList {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct PresetMod {}
