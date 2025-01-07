use crate::soundfont::pdta::{igen::InstrumentGen, imod::InstrumentMod};

pub struct PreInstrument {
    pub name: String,

    pub zones: Vec<PreZone>,
}

pub struct PreZone {
    /// Filename of the sample this voice uses
    pub sample: String,
}

impl PreZone {}
