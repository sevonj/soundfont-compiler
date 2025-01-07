//! Level 0 RIFF chunk 3/3 - sdta-list
//! Contains presets, instruments, sample headers

pub mod ibag;
pub mod igen;
pub mod imod;
pub mod inst;
pub mod pbag;
pub mod pgen;
pub mod phdr;
pub mod pmod;
pub mod shdr;

use ibag::InstBag;
use igen::InstGenList;
use imod::InstModList;
use riff::{ChunkContents, ChunkId};

use inst::Instruments;
use pbag::PresetBag;
use pgen::PresetGenList;
use phdr::PresetHeaders;
use pmod::PresetModList;
use shdr::SampleList;

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

impl From<PdtaList> for ChunkContents {
    fn from(value: PdtaList) -> Self {
        let contents: Vec<ChunkContents> = vec![
            value.phdr.into(),
            value.pbag.into(),
            value.pmod.into(),
            value.pgen.into(),
            value.inst.into(),
            value.ibag.into(),
            value.imod.into(),
            value.igen.into(),
            value.shdr.into(),
        ];

        ChunkContents::Children(riff::LIST_ID, ChunkId { value: *b"pdta" }, contents)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RangesType {
    pub lo: u8,
    pub hi: u8,
}

impl RangesType {
    pub fn as_bytes(&self) -> Vec<u8> {
        vec![self.lo, self.hi]
    }
}

#[derive(Debug, Clone)]
pub enum GenAmountType {
    Ranges(RangesType),
    Signed(i16),
    Unsigned(u16),
}

impl GenAmountType {
    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes = match self {
            GenAmountType::Ranges(value) => value.as_bytes(),
            GenAmountType::Signed(value) => value.to_le_bytes().to_vec(),
            GenAmountType::Unsigned(value) => value.to_le_bytes().to_vec(),
        };

        assert_eq!(bytes.len(), 2);

        bytes
    }
}
