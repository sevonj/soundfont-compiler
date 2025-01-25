use riff::{ChunkContents, ChunkId};

/// Modulator list for Presets and Instruments
#[derive(Debug, Clone)]
pub struct PresetModList {
    pub contents: Vec<PresetMod>,
}

impl Default for PresetModList {
    fn default() -> Self {
        Self::new()
    }
}

impl PresetModList {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }

    pub fn to_riff(&self) -> ChunkContents {
        let contents = vec![];
        //for gen in value.contents {
        //    contents.append(&mut gen.as_bytes());
        //}

        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"pmod" }, contents)
    }
}

#[derive(Debug, Clone)]
pub struct PresetMod {}
