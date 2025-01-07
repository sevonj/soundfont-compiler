//! Level 0 RIFF chunk 1/3 - INFO-list chunk
//! Contains metadata

use riff::{ChunkContents, ChunkId};

use super::sf_string_to_bytes;

#[derive(Debug, Clone, Copy)]
pub struct VersionTag {
    pub major: u16,
    pub minor: u16,
}

impl VersionTag {
    pub fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [self.major.to_le_bytes(), self.minor.to_le_bytes()].concat()
    }
}

/// Represents SoundFont 2 INFO Chunk.
#[derive(Debug, Clone)]
pub struct InfoList {
    /// SoundFont file version
    ifil: VersionTag,
    /// Sound engine. Use 'EMU8000'
    isng: String,
    /// Compatible bank e.g. 'General MIDI'
    inam: String,
    /*
    /// Wavetable ROM identifier
    irom: Option<String>,
    /// Wavetable ROM Version
    iver: Option<SfVersionTag>,
    /// Creation date in American date formatting e.g. 'December 6, 1917'
    icrd: Option<String>,
    ///
    ieng: Option<String>,
    */
    isft: Option<String>,
}

impl Default for InfoList {
    fn default() -> Self {
        Self {
            ifil: VersionTag::new(2, 1),
            isng: "EMU8000".to_owned(),
            inam: "General MIDI".to_owned(),

            isft: Some("SoundFont Compiler v0.0.0".into()),
        }
    }
}

impl From<InfoList> for ChunkContents {
    fn from(value: InfoList) -> Self {
        let mut contents = vec![
            ChunkContents::Data(ChunkId { value: *b"ifil" }, value.ifil.as_bytes()),
            ChunkContents::Data(ChunkId { value: *b"isng" }, sf_string_to_bytes(&value.inam)),
            ChunkContents::Data(ChunkId { value: *b"INAM" }, sf_string_to_bytes(&value.isng)),
        ];
        if let Some(isft) = &value.isft {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"ISFT" },
                sf_string_to_bytes(isft),
            ));
        }
        ChunkContents::Children(riff::LIST_ID, ChunkId { value: *b"INFO" }, contents)
    }
}
