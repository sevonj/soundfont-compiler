use serde::Deserialize;
use std::{collections::HashMap, path::Path};

use crate::soundfont::pdta::shdr::SampleHeader;

#[derive(Debug, Deserialize)]
pub struct PreSoundFont {
    pub authors: String,
    pub copyright: String,
    pub comments: String,

    pub presets: Vec<String>,
}

impl PreSoundFont {
    /// Read from a TOML file
    pub fn read<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(path).unwrap();
        toml::from_str(&contents).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct PrePreset {
    pub name: String,
    pub midi_preset: u16,
    pub midi_bank: u16,
    pub zones: HashMap<String, PrePresetZone>,
}

impl PrePreset {
    /// Read from a TOML file
    pub fn read<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(path).unwrap();
        toml::from_str(&contents).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct PrePresetZone {
    /// Filename of the instrument this zone uses
    pub instrument: String,
}

#[derive(Debug, Deserialize)]
pub struct PreInstrument {
    pub name: String,
    pub zones: HashMap<String, PreInstZone>,
}

impl PreInstrument {
    /// Read from a TOML file
    pub fn read<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(path).unwrap();
        toml::from_str(&contents).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct PreInstZone {
    /// Filename of the sample this voice uses
    pub sample: String,
    // pub start: u32,
    // pub end: u32,
    // pub startloop: u32,
    // pub endloop: u32,
    pub original_pitch: u8,
    pub pitch_correction: i8,
    // pub sample_link: u16,
    // pub sample_type: u16, //SFSampleLink enum
}

impl PreInstZone {}

pub struct PreSample {
    pub data: Vec<u16>,
    pub header: SampleHeader,
}
