//! Level 0 RIFF chunk 3/3 - sdta-list
//! Contains presets, instruments, sample headers

mod ibag;
mod igen;
mod imod;
mod inst;
mod pbag;
mod pgen;
mod phdr;
mod pmod;
mod shdr;

use riff::{ChunkContents, ChunkId};

pub use ibag::{InstBag, InstZone};
pub use igen::{InstGenList, InstrumentGen};
pub use imod::InstModList;
pub use inst::{Instrument, Instruments};
pub use pbag::{PresetBag, PresetZone};
pub use pgen::{PresetGen, PresetGenList};
pub use phdr::{PresetHeader, PresetHeaders};
pub use pmod::PresetModList;
pub use shdr::{SampleHeader, SampleList};

/// The Hydra
///
/// Rough hierarchy based on pointers:
/// ```
/// pdta
/// ├── phdr
/// │   └── pbag
/// │       ├── pmod
/// │       └── pgen
/// ├── inst
/// │   └── ibag
/// │       ├── imod
/// │       └── igen
/// └── shdr
/// ```
#[derive(Debug, Clone)]
pub struct PdtaList {
    pub phdr: PresetHeaders, // Preset
    pub pbag: PresetBag,     // Preset Zone
    pub pmod: PresetModList, // Preset Modulators
    pub pgen: PresetGenList, //
    pub inst: Instruments,   // Instrument
    pub ibag: InstBag,       // Instrument Zone
    pub imod: InstModList,   // Instrument Modulators
    pub igen: InstGenList,   //
    pub shdr: SampleList,    // Sample
}

impl Default for PdtaList {
    fn default() -> Self {
        Self {
            phdr: PresetHeaders::new(),
            pbag: PresetBag::new(),
            pmod: PresetModList::new(),
            pgen: PresetGenList::new(),

            inst: Instruments::new(),
            ibag: InstBag::new(),
            imod: InstModList::new(),
            igen: InstGenList::new(),

            shdr: SampleList::new(),
        }
    }
}

impl PdtaList {
    pub fn to_riff(&self) -> ChunkContents {
        let contents: Vec<ChunkContents> = vec![
            self.phdr.to_riff(),
            self.pbag.to_riff(),
            self.pmod.to_riff(),
            self.pgen.to_riff(),
            self.inst.to_riff(),
            self.ibag.to_riff(),
            self.imod.to_riff(),
            self.igen.to_riff(),
            self.shdr.to_riff(),
        ];

        ChunkContents::Children(riff::LIST_ID, ChunkId { value: *b"pdta" }, contents)
    }
}

/*
#[derive(Debug, Clone, Copy)]
pub struct RangesType {
    pub lo: u8,
    pub hi: u8,
}

impl RangesType {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.lo, self.hi]
    }
}*/

/// 16-bit types for generator values
#[derive(Debug, Clone)]
pub enum GenAmountType {
    Range { lo: u8, hi: u8 },
    Signed(i16),
    Unsigned(u16),
}

impl GenAmountType {
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = match self {
            GenAmountType::Range { lo, hi } => vec![*lo, *hi],
            GenAmountType::Signed(value) => value.to_le_bytes().to_vec(),
            GenAmountType::Unsigned(value) => value.to_le_bytes().to_vec(),
        };

        assert_eq!(bytes.len(), 2);

        bytes
    }
}
