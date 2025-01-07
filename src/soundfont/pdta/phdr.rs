use riff::{ChunkContents, ChunkId};

#[derive(Debug, Clone)]
pub struct PresetHeaders {
    pub contents: Vec<PresetHeader>,
}

impl From<PresetHeaders> for ChunkContents {
    fn from(value: PresetHeaders) -> Self {
        println!("Packing phdr: {value:?}");

        let mut contents = vec![];
        for header in value.contents {
            contents.append(&mut header.as_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 38, 0);

        ChunkContents::Data(ChunkId { value: *b"phdr" }, contents)
    }
}

impl PresetHeaders {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
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

impl Default for PresetHeader {
    fn default() -> Self {
        Self {
            name: Default::default(),
            preset: Default::default(),
            bank: Default::default(),
            pbag_idx: Default::default(),
            library: 0,
            genre: 0,
            morphology: 0,
        }
    }
}

impl PresetHeader {
    pub fn as_bytes(&self) -> Vec<u8> {
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
