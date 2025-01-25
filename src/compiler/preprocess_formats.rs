use chrono::Utc;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

use crate::soundfont::{
    info::{InfoList, VersionTag},
    pdta::SampleHeader,
    SoundfontError,
};

use super::CompilerError;

#[derive(Debug, Deserialize)]
pub struct PreSoundFont {
    //pub soundfont_version: Option<String>
    /// Target sound engine. Defaults to "EMU8000"
    pub sound_engine: Option<String>,
    /// Name of the soundfont
    pub name: String,
    // /// Wavetable ROM identifier
    // irom: Option<String>,
    // /// Wavetable ROM version
    // iver: Option<VersionTag>,
    /// Soundfont author credits
    pub authors: Option<String>,
    /// Target product of this soundfont e.g “SBAWE32”
    pub target_product: Option<String>,
    /// Copyright message
    pub copyright: Option<String>,
    /// Soundfont comment or description
    pub comments: Option<String>,

    /// List of preset filepaths, relative to the presets directory
    pub presets: Vec<String>,
}

impl PreSoundFont {
    /// Read from a TOML manifest
    pub fn read<P>(filepath: P) -> Result<Self, CompilerError>
    where
        P: AsRef<Path>,
    {
        let Ok(contents) = std::fs::read_to_string(filepath) else {
            return Err(CompilerError::ProjectManifestCantOpen);
        };
        let Ok(this) = toml::from_str(&contents) else {
            return Err(CompilerError::ProjectManifestCantOpen);
        };
        Ok(this)
    }

    pub fn generate_infolist(&self) -> Result<InfoList, SoundfontError> {
        let mut info = InfoList::default();

        info.set_ifil(VersionTag::new(2, 4));
        info.set_isng("EMU8000".into())?;
        info.set_inam(self.name.clone())?;
        //info.set_irom()
        //info.set_iver()
        let now = Utc::now();
        //let month = match now.month0() {
        //    0 => "January",
        //    1 => "February",
        //    2 => "March",
        //    3 => "April",
        //    4 => "May",
        //    5 => "June",
        //    6 => "July",
        //    7 => "August",
        //    8 => "September",
        //    9 => "October",
        //    10 => "November",
        //    11 => "December",
        //    _ => panic!("too many months"),
        //};
        //panic!("{month} {}, {}", now.day(), now.year());
        info.set_icrd(Some(now.to_rfc3339()))?;
        info.set_ieng(self.authors.clone())?;
        info.set_iprd(self.target_product.clone())?;
        info.set_icop(self.copyright.clone())?;
        info.set_icmt(self.comments.clone())?;
        info.set_isft(Some(
            "SoundFont Compiler v0.0.0:SoundFont Compiler v0.0.0".into(),
        ))?;

        Ok(info)
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
    // Loop Mode
    pub sample_modes: Option<i16>,
    ///
    pub mod_lfo_to_pitch: Option<i16>,
    pub vib_lfo_to_pitch: Option<i16>,
    pub mod_env_to_pitch: Option<i16>,
    pub initial_filter_fc: Option<i16>,
    pub initial_filter_q: Option<i16>,
    pub mod_lfo_to_filter_fc: Option<i16>,
    pub mod_env_to_filter_fc: Option<i16>,
    pub mod_lfo_to_volume: Option<i16>,
    pub chorus_effects_send: Option<i16>,
    pub reverb_effects_send: Option<i16>,
    pub pan: Option<i16>,
    pub freq_mod_lfo: Option<i16>,
    pub delay_vib_lfo: Option<i16>,
    pub freq_vib_lfo: Option<i16>,
    pub delay_mod_env: Option<i16>,
    pub attack_mod_env: Option<i16>,
    pub hold_mod_env: Option<i16>,
    pub decay_mod_env: Option<i16>,
    pub sustain_mod_env: Option<i16>,
    pub release_mod_env: Option<i16>,
    pub keynum_to_mod_env_hold: Option<i16>,
    pub delay_vol_env: Option<i16>,
    pub attack_vol_env: Option<i16>,
    pub hold_vol_env: Option<i16>,
    pub decay_vol_env: Option<i16>,
    pub sustain_vol_env: Option<i16>,
    pub release_vol_env: Option<i16>,
    pub keynum_to_vol_env_hold: Option<i16>,
    pub keynum_to_vol_env_decay: Option<i16>,
    pub key_range: Option<i16>,
    pub vel_range: Option<i16>,
    pub keynum: Option<i16>,
    pub velocity: Option<i16>,
    pub initial_attenuation: Option<i16>,
    pub coarse_tune: Option<i16>,
    pub fine_tune: Option<i16>,
    pub scale_tuning: Option<i16>,
    pub exclusive_class: Option<i16>,
    pub overriding_root_key: Option<i16>,
}

impl PreInstZone {}
