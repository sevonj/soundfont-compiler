pub mod preprocess_formats;

use std::path::PathBuf;

use wavers::Wav;

use crate::soundfont::{
    pdta::{
        ibag::InstZone, igen::InstrumentGen, inst::Instrument, pbag::PresetZone, pgen::PresetGen,
        phdr::PresetHeader, shdr::SampleHeader, GenAmountType,
    },
    SoundFont2,
};

pub struct FontData {
    preset_headers: Vec<PresetHeader>,
    preset_zones: Vec<PresetZone>,
    preset_gens: Vec<PresetGen>,

    inst_headers: Vec<Instrument>,
    inst_zones: Vec<InstZone>,
    inst_gens: Vec<InstrumentGen>,

    sample_headers: Vec<SampleHeader>,
    sample_data: Vec<Vec<i16>>,
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

            sample_headers: vec![],
            sample_data: vec![],
        }
    }

    pub fn load(&mut self) {
        // PHDR
        self.preset_headers.push(PresetHeader {
            name: "Preset".into(),
            preset: 0,
            bank: 0,
            pbag_idx: 0,
            ..Default::default()
        });
        // pbag
        self.preset_zones.push(PresetZone::new(0, 0));
        //self.preset_zones.push(PresetZone::new(1, 0));

        // pmod

        // Pgen
        self.preset_gens.push(PresetGen {
            sf_gen_oper: 41,                        // instrument
            gen_amount: GenAmountType::Unsigned(0), // First instrument
        });

        // INST
        self.inst_headers.push(Instrument {
            name: "Example-Instrument".into(),
            inst_bag_idx: 0,
        });

        // ibag
        self.inst_zones.push(InstZone::new(0, 0));
        //self.inst_zones.push(InstZone::new(1, 0));

        // imod

        // igen
        self.inst_gens.push(InstrumentGen {
            sf_gen_oper: 53,                        // Sample id
            gen_amount: GenAmountType::Unsigned(0), // First
        });

        // Samples
        let samples = vec![
            "daytona_doo001",
            "daytona_doo002",
            "daytona_doo003",
            "daytona_doo004",
        ];
        for sample in samples {
            self.add_sample(sample);
        }
    }

    pub fn generate_soundfont(&mut self) -> SoundFont2 {
        let mut soundfont = SoundFont2::new();

        // --- INFO

        // --- sdta
        assert_eq!(self.sample_headers.len(), self.sample_data.len());
        for i in 0..self.sample_headers.len() {
            let sample = &self.sample_data[i];
            let header = &mut self.sample_headers[i];

            // Header offsets were relative until now.
            let position = soundfont.sdta.smpl.len();
            header.start += position as u32;
            header.end += position as u32;
            header.startloop += position as u32;
            header.endloop += position as u32;

            soundfont.sdta.smpl.append(&mut sample.clone());
            soundfont.sdta.smpl.append(&mut vec![0, 46]); // 46 or more points padding required
        }

        // --- pdta
        // --- pdta: phdr
        for header in &self.preset_headers {
            soundfont.pdta.phdr.contents.push(header.clone());
        }
        let term_phdr = PresetHeader {
            name: "EOI".into(),
            preset: 0,
            bank: 0,
            pbag_idx: self.preset_zones.len() as u16,
            ..Default::default()
        };
        soundfont.pdta.phdr.contents.push(term_phdr);

        // --- pdta: pbag
        for zone in &self.preset_zones {
            soundfont.pdta.pbag.contents.push(zone.clone());
        }
        let term_pzone = PresetZone::new(1, 0);
        soundfont.pdta.pbag.contents.push(term_pzone);

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
        let term_inst = Instrument {
            name: "EOI".into(),
            inst_bag_idx: self.inst_zones.len() as u16,
        };
        soundfont.pdta.inst.contents.push(term_inst);

        // --- pdta: ibag
        for zone in &self.inst_zones {
            soundfont.pdta.ibag.contents.push(zone.clone());
        }
        let term_izone = InstZone::new(1, 0);
        soundfont.pdta.ibag.contents.push(term_izone);

        // --- pdta: imod

        // --- pdta: igen
        for igen in &self.inst_gens {
            soundfont.pdta.igen.contents.push(igen.clone());
        }
        soundfont.pdta.igen.contents.push(InstrumentGen::terminal());

        // --- pdta: shdr
        for header in &self.sample_headers {
            soundfont.pdta.shdr.contents.push(header.clone());
        }
        soundfont.pdta.shdr.contents.push(SampleHeader::terminal());

        soundfont
    }

    fn add_sample(&mut self, name: &str) {
        let path = PathBuf::from(format!("example/samples/{name}.wav"));
        let mut wav: Wav<i16> = Wav::from_path(path).unwrap();
        let data = wav.read().unwrap().to_vec();
        let len = data.len() as u32;

        // Mono only for now.
        assert_eq!(wav.n_channels(), 1);

        let header = SampleHeader {
            name: name.into(),
            start: 0,
            end: len,
            startloop: 8,
            endloop: len - 8,
            sample_rate: wav.sample_rate() as u32,
            original_pitch: 64,
            pitch_correction: 0,
            sample_link: 0,
            sample_type: 1,
        };

        self.sample_headers.push(header);
        self.sample_data.push(data);
    }
}
