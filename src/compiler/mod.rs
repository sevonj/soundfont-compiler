mod error;
mod preprocess_formats;

use std::path::PathBuf;
use wavers::Wav;

use crate::soundfont::pdta::{
    GenAmountType, InstZone, Instrument, InstrumentGen, PdtaList, PresetGen, PresetHeader,
    PresetZone, SampleHeader,
};
use crate::soundfont::{info::InfoList, sdta::SdtaList, SoundFont2};
pub use error::CompilerError;
use preprocess_formats::{PreInstrument, PrePreset, PreSoundFont};

#[derive(Debug, Clone)]
pub struct SampleWrap {
    header: SampleHeader,
    data: Vec<i16>,
}

#[derive(Debug)]
pub struct FontData {
    info_list: Option<InfoList>,

    preset_headers: Vec<PresetHeader>,
    preset_zones: Vec<PresetZone>,
    preset_gens: Vec<PresetGen>,

    inst_headers: Vec<Instrument>,
    inst_zones: Vec<InstZone>,
    inst_gens: Vec<InstrumentGen>,

    samples: Vec<SampleWrap>,
}

impl FontData {
    pub fn read(filepath: &str) -> Result<Self, CompilerError> {
        let pre_info = PreSoundFont::read(filepath)?;

        let info_list = pre_info.generate_infolist().ok();

        let mut preset_headers: Vec<PresetHeader> = vec![];
        let mut preset_zones: Vec<PresetZone> = vec![];
        let mut preset_gens: Vec<PresetGen> = vec![];

        let mut inst_headers: Vec<Instrument> = vec![];
        let mut inst_zones: Vec<InstZone> = vec![];
        let mut inst_gens: Vec<InstrumentGen> = vec![];

        let mut samples: Vec<SampleWrap> = vec![];

        for preset_filename in pre_info.presets {
            let pre_preset = PrePreset::read(format!("example_project/presets/{preset_filename}"));
            let pbag_idx = inst_zones.len() as u16;

            for (_pzone_name, pre_pzone) in pre_preset.zones {
                let path = PathBuf::from(format!(
                    "example_project/instruments/{}",
                    pre_pzone.instrument
                ));
                let inst = PreInstrument::read(path);

                let inst_bag_idx = inst_zones.len() as u16;

                for (_izone_name, pre_izone) in inst.zones {
                    let pre_zone = pre_izone;
                    let path =
                        PathBuf::from(format!("example_project/samples/{}", pre_zone.sample));
                    let mut wav: Wav<i16> = Wav::from_path(path).unwrap();
                    assert_eq!(wav.n_channels(), 1); // Mono only for now

                    let sample_data = wav.read().unwrap().to_vec();
                    let sample_header = SampleHeader {
                        name: pre_zone.sample.clone(),
                        start: 0,
                        end: sample_data.len() as u32,
                        startloop: 8,
                        endloop: sample_data.len() as u32 - 8,
                        sample_rate: wav.sample_rate() as u32,
                        original_pitch: pre_zone.original_pitch,
                        pitch_correction: pre_zone.pitch_correction,
                        sample_link: 0,
                        sample_type: 1,
                    };
                    samples.push(SampleWrap {
                        header: sample_header,
                        data: sample_data,
                    });

                    inst_gens.push(InstrumentGen {
                        sf_gen_oper: 53, // Sample id
                        gen_amount: GenAmountType::Unsigned(samples.len() as u16 - 1),
                    });

                    inst_zones.push(InstZone {
                        gen_idx: inst_gens.len() as u16 - 1,
                        mod_idx: 0, //self.inst_mods.len(),
                    });
                }

                inst_headers.push(Instrument {
                    name: inst.name,
                    inst_bag_idx,
                });

                preset_gens.push(PresetGen {
                    sf_gen_oper: 41, // Inst id
                    gen_amount: GenAmountType::Unsigned(inst_headers.len() as u16 - 1),
                });

                preset_zones.push(PresetZone {
                    gen_idx: preset_gens.len() as u16 - 1,
                    mod_idx: 0, //self.inst_mods.len(),
                });
            }

            preset_headers.push(PresetHeader {
                name: pre_preset.name,
                preset: pre_preset.midi_preset,
                bank: pre_preset.midi_bank,
                pbag_idx,
                library: 0,
                genre: 0,
                morphology: 0,
            });
        }

        Ok(Self {
            info_list,

            preset_headers,
            preset_zones,
            preset_gens,

            inst_headers,
            inst_zones,
            inst_gens,

            samples,
        })
    }

    /// TODO: Make un-mut
    pub fn generate_soundfont(&mut self) -> SoundFont2 {
        let info = self.info_list.as_ref().unwrap().clone();

        let mut sdta = SdtaList::default();

        for sample in &mut self.samples {
            let data = &sample.data;
            let header = &mut sample.header;

            // Header offsets were relative until now.
            let position = sdta.smpl.len();
            header.start += position as u32;
            header.end += position as u32;
            header.startloop += position as u32;
            header.endloop += position as u32;

            sdta.smpl.append(&mut data.clone());
            sdta.smpl.append(&mut vec![0, 46]); // 46 or more points padding required
        }

        // --- pdta
        // --- pdta: phdr
        let mut pdta = PdtaList::default();
        for header in &self.preset_headers {
            pdta.phdr.contents.push(header.clone());
        }
        pdta.phdr.contents.push(PresetHeader {
            name: "EOI".into(),
            preset: 0,
            bank: 0,
            pbag_idx: self.preset_zones.len() as u16,
            ..Default::default()
        });

        // --- pdta: pbag
        for zone in &self.preset_zones {
            pdta.pbag.contents.push(zone.clone());
        }
        pdta.pbag
            .contents
            .push(PresetZone::new(self.preset_gens.len() as u16, 0));

        // --- pdta: pmod

        // --- pdta: pgen
        for pgen in &self.preset_gens {
            pdta.pgen.contents.push(pgen.clone());
        }
        pdta.pgen.contents.push(PresetGen::terminal());

        // --- pdta: inst
        for inst in &self.inst_headers {
            pdta.inst.contents.push(inst.clone());
        }
        pdta.inst.contents.push(Instrument {
            name: "EOI".into(),
            inst_bag_idx: self.inst_zones.len() as u16,
        });

        // --- pdta: ibag
        for zone in &self.inst_zones {
            pdta.ibag.contents.push(zone.clone());
        }
        pdta.ibag
            .contents
            .push(InstZone::new(self.inst_gens.len() as u16, 0));

        // --- pdta: imod

        // --- pdta: igen
        for igen in &self.inst_gens {
            pdta.igen.contents.push(igen.clone());
        }
        pdta.igen.contents.push(InstrumentGen::terminal());

        // --- pdta: shdr
        for sample in &self.samples {
            pdta.shdr.contents.push(sample.header.clone());
        }
        pdta.shdr.contents.push(SampleHeader::terminal());

        SoundFont2::new(info, sdta, pdta)
    }
}
