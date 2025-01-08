pub mod preprocess_formats;

use std::path::PathBuf;

use preprocess_formats::{PreInstrument, PrePreset, PreSoundFont};
use wavers::Wav;

use crate::soundfont::{
    pdta::{
        ibag::InstZone, igen::InstrumentGen, inst::Instrument, pbag::PresetZone, pgen::PresetGen,
        phdr::PresetHeader, shdr::SampleHeader, GenAmountType,
    },
    SoundFont2,
};

#[derive(Debug, Clone)]
pub struct SampleWrap {
    header: SampleHeader,
    data: Vec<i16>,
}

#[derive(Debug)]
pub struct FontData {
    preset_headers: Vec<PresetHeader>,
    preset_zones: Vec<PresetZone>,
    preset_gens: Vec<PresetGen>,

    inst_headers: Vec<Instrument>,
    inst_zones: Vec<InstZone>,
    inst_gens: Vec<InstrumentGen>,

    samples: Vec<SampleWrap>,
}

impl FontData {
    pub fn new() -> Self {
        Self {
            preset_headers: vec![],
            preset_zones: vec![],
            preset_gens: vec![],

            inst_headers: vec![],
            inst_zones: vec![],
            inst_gens: vec![],

            samples: vec![],
        }
    }

    pub fn load(&mut self) {
        let pre_info = PreSoundFont::read("example_project/SoundFont.toml");

        for preset_filename in pre_info.presets {
            let pre_preset = PrePreset::read(format!("example_project/presets/{preset_filename}"));
            let pbag_idx = self.inst_zones.len() as u16;

            for (_pzone_name, pre_pzone) in pre_preset.zones {
                let path = PathBuf::from(format!(
                    "example_project/instruments/{}",
                    pre_pzone.instrument
                ));
                let inst = PreInstrument::read(path);

                let inst_bag_idx = self.inst_zones.len() as u16;

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
                    self.samples.push(SampleWrap {
                        header: sample_header,
                        data: sample_data,
                    });

                    self.inst_gens.push(InstrumentGen {
                        sf_gen_oper: 53, // Sample id
                        gen_amount: GenAmountType::Unsigned(self.samples.len() as u16 - 1),
                    });

                    self.inst_zones.push(InstZone {
                        gen_idx: self.inst_gens.len() as u16 - 1,
                        mod_idx: 0, //self.inst_mods.len(),
                    });
                }

                self.inst_headers.push(Instrument {
                    name: inst.name,
                    inst_bag_idx,
                });

                self.preset_gens.push(PresetGen {
                    sf_gen_oper: 41, // Inst id
                    gen_amount: GenAmountType::Unsigned(self.inst_headers.len() as u16 - 1),
                });

                self.preset_zones.push(PresetZone {
                    gen_idx: self.preset_gens.len() as u16 - 1,
                    mod_idx: 0, //self.inst_mods.len(),
                });
            }

            self.preset_headers.push(PresetHeader {
                name: pre_preset.name,
                preset: pre_preset.midi_preset,
                bank: pre_preset.midi_bank,
                pbag_idx,
                library: 0,
                genre: 0,
                morphology: 0,
            });
        }
    }

    pub fn generate_soundfont(&mut self) -> SoundFont2 {
        let mut soundfont = SoundFont2::new();

        // --- INFO

        // --- sdta
        for sample in &mut self.samples {
            let data = &sample.data;
            let header = &mut sample.header;

            // Header offsets were relative until now.
            let position = soundfont.sdta.smpl.len();
            header.start += position as u32;
            header.end += position as u32;
            header.startloop += position as u32;
            header.endloop += position as u32;

            soundfont.sdta.smpl.append(&mut data.clone());
            soundfont.sdta.smpl.append(&mut vec![0, 46]); // 46 or more points padding required
        }

        // --- pdta
        // --- pdta: phdr
        for header in &self.preset_headers {
            soundfont.pdta.phdr.contents.push(header.clone());
        }
        soundfont.pdta.phdr.contents.push(PresetHeader {
            name: "EOI".into(),
            preset: 0,
            bank: 0,
            pbag_idx: self.preset_zones.len() as u16,
            ..Default::default()
        });

        // --- pdta: pbag
        for zone in &self.preset_zones {
            soundfont.pdta.pbag.contents.push(zone.clone());
        }
        soundfont
            .pdta
            .pbag
            .contents
            .push(PresetZone::new(self.preset_gens.len() as u16, 0));

        // --- pdta: pmod

        // --- pdta: pgen
        for pgen in &self.preset_gens {
            soundfont.pdta.pgen.contents.push(pgen.clone());
        }
        soundfont.pdta.pgen.contents.push(PresetGen::terminal());

        // --- pdta: inst
        for inst in &self.inst_headers {
            soundfont.pdta.inst.contents.push(inst.clone());
        }
        soundfont.pdta.inst.contents.push(Instrument {
            name: "EOI".into(),
            inst_bag_idx: self.inst_zones.len() as u16,
        });

        // --- pdta: ibag
        for zone in &self.inst_zones {
            soundfont.pdta.ibag.contents.push(zone.clone());
        }
        soundfont
            .pdta
            .ibag
            .contents
            .push(InstZone::new(self.inst_gens.len() as u16, 0));

        // --- pdta: imod

        // --- pdta: igen
        for igen in &self.inst_gens {
            soundfont.pdta.igen.contents.push(igen.clone());
        }
        soundfont.pdta.igen.contents.push(InstrumentGen::terminal());

        // --- pdta: shdr
        for sample in &self.samples {
            soundfont.pdta.shdr.contents.push(sample.header.clone());
        }
        soundfont.pdta.shdr.contents.push(SampleHeader::terminal());

        soundfont
    }
}
