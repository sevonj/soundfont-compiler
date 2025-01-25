use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone)]
pub struct PresetHeaders {
    pub contents: Vec<PresetHeader>,
}

impl Default for PresetHeaders {
    fn default() -> Self {
        Self::new()
    }
}

impl PresetHeaders {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }

    pub fn to_riff(&self) -> ChunkContents {
        let mut contents = vec![];
        for header in &self.contents {
            contents.append(&mut header.to_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 38, 0);

        ChunkContents::Data(ChunkId { value: *b"phdr" }, contents)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PresetHeader {
    /// Must be unique, ASCII, and at most 256B long.
    pub name: String,
    /// MIDI Preset number. Bank/Preset combination must be unique.
    pub preset: u16,
    /// MIDI Bank number. Bank/Preset combination must be unique.
    pub bank: u16,
    /// Index in preset bag
    pub pbag_idx: u16,
    /// Unused
    pub library: u32,
    /// Unused
    pub genre: u32,
    /// Unused
    pub morphology: u32,
}

impl PresetHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        let mut name_bytes = self.name.as_bytes().to_vec();
        assert!(name_bytes.len() <= 20);
        name_bytes.resize(20, 0);

        bytes.append(&mut name_bytes);
        bytes.append(&mut self.preset.to_le_bytes().to_vec());
        bytes.append(&mut self.bank.to_le_bytes().to_vec());
        bytes.append(&mut self.pbag_idx.to_le_bytes().to_vec());
        bytes.append(&mut self.library.to_le_bytes().to_vec());
        bytes.append(&mut self.genre.to_le_bytes().to_vec());
        bytes.append(&mut self.morphology.to_le_bytes().to_vec());

        assert_eq!(bytes.len(), 38);
        bytes
    }
}
