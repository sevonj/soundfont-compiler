//! Level 0 RIFF chunk 1/3 - INFO-list chunk
//! Contains metadata

mod version_tag;

use riff::{ChunkContents, ChunkId};

use super::{string_to_bytes, SoundfontError};
pub use version_tag::VersionTag;

/// Represents SoundFont 2 INFO Chunk.
#[derive(Debug, Clone)]
pub struct InfoList {
    /// SoundFont spec version
    ifil: VersionTag,
    /// Target sound engine. If in doubt, use "EMU8000"
    isng: String,
    /// Name of the soundfont
    inam: String,
    /// Wavetable ROM identifier
    irom: Option<String>,
    /// Wavetable ROM version
    iver: Option<VersionTag>,
    /// Creation date in American formatting e.g. "December 6, 1917"
    icrd: Option<String>,
    /// Soundfont author credits
    ieng: Option<String>,
    /// Target product of this soundfont e.g “SBAWE32”
    iprd: Option<String>,
    /// Copyright message
    icop: Option<String>,
    /// Soundfont comment or description
    icmt: Option<String>,
    /// Name of the software used to create the soundfont e.g. "SoundFont Compiler v0.0.0"
    isft: Option<String>,
}

impl Default for InfoList {
    fn default() -> Self {
        Self {
            ifil: VersionTag::new(2, 4),
            isng: "EMU8000".into(),
            inam: "A New Soundfont".into(),
            irom: None,
            iver: None,
            icrd: None,
            ieng: None,
            iprd: None,
            icop: None,
            icmt: None,
            isft: None,
        }
    }
}

impl InfoList {
    pub fn to_riff(&self) -> ChunkContents {
        let mut contents = vec![
            ChunkContents::Data(ChunkId { value: *b"ifil" }, self.ifil.to_bytes()),
            ChunkContents::Data(ChunkId { value: *b"isng" }, string_to_bytes(&self.inam)),
            ChunkContents::Data(ChunkId { value: *b"INAM" }, string_to_bytes(&self.isng)),
        ];
        if let Some(irom) = &self.irom {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"irom" },
                string_to_bytes(irom),
            ));
        }
        if let Some(iver) = &self.iver {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"iver" },
                iver.to_bytes(),
            ));
        }
        if let Some(icrd) = &self.icrd {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"ICRD" },
                string_to_bytes(icrd),
            ));
        }
        if let Some(ieng) = &self.ieng {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"IENG" },
                string_to_bytes(ieng),
            ));
        }
        if let Some(iprd) = &self.iprd {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"IPRD" },
                string_to_bytes(iprd),
            ));
        }
        if let Some(icop) = &self.icop {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"ICOP" },
                string_to_bytes(icop),
            ));
        }
        if let Some(icmt) = &self.icmt {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"ICMT" },
                string_to_bytes(icmt),
            ));
        }
        if let Some(isft) = &self.isft {
            contents.push(ChunkContents::Data(
                ChunkId { value: *b"ISFT" },
                string_to_bytes(isft),
            ));
        }

        ChunkContents::Children(riff::LIST_ID, ChunkId { value: *b"INFO" }, contents)
    }
}

impl InfoList {
    /// SoundFont spec version
    pub fn ifil(&self) -> &VersionTag {
        &self.ifil
    }

    /// Target sound engine. If in doubt, use 'EMU8000'
    pub fn isng(&self) -> &String {
        &self.isng
    }

    /// Name of the soundfont
    pub fn inam(&self) -> &String {
        &self.inam
    }

    /// Wavetable ROM identifier
    pub fn irom(&self) -> Option<&str> {
        self.irom.as_deref()
    }

    /// Wavetable ROM version
    pub fn iver(&self) -> Option<&VersionTag> {
        self.iver.as_ref()
    }

    /// Creation date in American formatting e.g. "December 6, 1917"
    pub fn icrd(&self) -> Option<&str> {
        self.icrd.as_deref()
    }

    /// Soundfont author credits
    pub fn ieng(&self) -> Option<&str> {
        self.ieng.as_deref()
    }

    /// Target product of this soundfont e.g “SBAWE32”
    pub fn iprd(&self) -> Option<&str> {
        self.iprd.as_deref()
    }

    /// Copyright message
    pub fn icop(&self) -> Option<&str> {
        self.icop.as_deref()
    }

    /// Soundfont comment or description
    pub fn icmt(&self) -> Option<&str> {
        self.icmt.as_deref()
    }

    /// Name of the software used to create the soundfont, and the software most recently used to
    /// edit it. e.g. "SoundFont Compiler v0.0.0:SoundFont Editor v0.0.0"
    pub fn isft(&self) -> Option<&str> {
        self.isft.as_deref()
    }

    /// You likely want to use version "2.04"
    pub fn set_ifil(&mut self, version: VersionTag) {
        self.ifil = version
    }

    /// String must be ascii and fit into 256 bytes.
    pub fn set_isng(&mut self, engine: String) -> Result<(), SoundfontError> {
        validate_string(&engine, 256)?;
        self.isng = engine;
        Ok(())
    }

    /// String must be ascii and fit into 256 bytes.
    pub fn set_inam(&mut self, name: String) -> Result<(), SoundfontError> {
        validate_string(&name, 256)?;
        self.inam = name;
        Ok(())
    }

    /// String must be ascii and fit into 256 bytes.
    pub fn set_irom(&mut self, rom_name: Option<String>) -> Result<(), SoundfontError> {
        if let Some(value) = &rom_name {
            validate_string(value, 256)?;
        }
        self.irom = rom_name;
        Ok(())
    }

    pub fn set_iver(&mut self, rom_version: Option<VersionTag>) {
        self.iver = rom_version;
    }

    /// String must be ascii and fit into 256 bytes.
    pub fn set_icrd(&mut self, creation_date: Option<String>) -> Result<(), SoundfontError> {
        if let Some(value) = &creation_date {
            validate_string(value, 256)?;
        }
        self.icrd = creation_date;
        Ok(())
    }

    /// String must be ascii and fit into 256 bytes.
    pub fn set_ieng(&mut self, authors: Option<String>) -> Result<(), SoundfontError> {
        if let Some(value) = &authors {
            validate_string(value, 256)?;
        }
        self.ieng = authors;
        Ok(())
    }

    /// String must be ascii and fit into 256 bytes.
    pub fn set_iprd(&mut self, target_product: Option<String>) -> Result<(), SoundfontError> {
        if let Some(value) = &target_product {
            validate_string(value, 256)?;
        }
        self.iprd = target_product;
        Ok(())
    }

    /// String must be ascii and fit into 256 bytes.
    pub fn set_icop(&mut self, copyright: Option<String>) -> Result<(), SoundfontError> {
        if let Some(value) = &copyright {
            validate_string(value, 256)?;
        }
        self.icop = copyright;
        Ok(())
    }

    /// String must be ascii and fit into 65_536 bytes.
    pub fn set_icmt(&mut self, comments: Option<String>) -> Result<(), SoundfontError> {
        if let Some(value) = &comments {
            validate_string(value, 65536)?;
        }
        self.icmt = comments;
        Ok(())
    }

    /// String must be ascii and fit into 256 bytes.
    pub fn set_isft(&mut self, software_name: Option<String>) -> Result<(), SoundfontError> {
        if let Some(value) = &software_name {
            validate_string(value, 256)?;
        }
        self.isft = software_name;
        Ok(())
    }
}

/// Makes sure string is ASCII and fits into size constraint.
pub fn validate_string(value: &str, limit: usize) -> Result<(), SoundfontError> {
    if !value.is_ascii() {
        return Err(SoundfontError::StringNonAscii);
    }
    let len = value.as_bytes().len();
    if len > limit {
        return Err(SoundfontError::StringLimit { limit, len });
    }
    Ok(())
}
