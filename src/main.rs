pub mod compiler;
pub mod soundfont;

use std::fs::File;

use compiler::FontData;

fn main() {
    let mut fontdata = FontData::new();
    fontdata.load();

    let soundfont = fontdata.generate_soundfont();
    let riff = soundfont.create_riff();

    let mut file = File::create("test_output.sf2").expect("filecreatefail");
    riff.write(&mut file).expect("fail");

    let mut open_file = File::open("test_output.sf2").unwrap();
    // open_file = File::open("SM64SF_V2.sf2").unwrap();

    println!("Result:");
    let font;
    match rustysynth::SoundFont::new(&mut open_file) {
        Ok(val) => {
            println!("was ok");
            font = val
        }
        Err(e) => {
            println!("ERR: {e}");
            return;
        }
    }

    for preset in font.get_presets() {
        log_preset(preset);
    }

    for inst in font.get_instruments() {
        log_instrument(inst);
    }

    for sample in font.get_sample_headers() {
        log_sample(sample);
    }

    //println!("{:02x?}", font.get_wave_data())
}

fn log_sample(sample: &rustysynth::SampleHeader) {
    print!("Sample - ");
    println!("{}", sample.get_name());
    println!("  start: {}", sample.get_start());
    println!("  end: {}", sample.get_end());
    println!("  start_loop: {}", sample.get_start_loop());
    println!("  end_loop: {}", sample.get_end_loop());
    println!("  sample_rate: {}", sample.get_sample_rate());
    println!("  original_pitch: {}", sample.get_original_pitch());
    println!("  pitch_correction: {}", sample.get_pitch_correction());
    println!("  link: {}", sample.get_link());
    println!("  sample_type: {}", sample.get_sample_type());
}
fn log_instrument(inst: &rustysynth::Instrument) {
    print!("Instrument - ");
    println!("{}", inst.get_name());
    println!("  Zones: {}", inst.get_regions().len());
    for region in inst.get_regions() {
        println!("      i-zone: ");
        println!("          sample_id: {}", region.get_sample_id());
        println!("          sample_start: {}", region.get_sample_start());
        println!("          sample_end: {}", region.get_sample_end());
        println!(
            "          sample_start_loop: {}",
            region.get_sample_start_loop()
        );
        println!(
            "          sample_end_loop: {}",
            region.get_sample_end_loop()
        );
    }
}
fn log_preset(preset: &rustysynth::Preset) {
    print!("Preset - ");
    println!("{}", preset.get_name());
    println!("  Zones: {}", preset.get_regions().len());
    for region in preset.get_regions() {
        println!("      p-zone: ");
        println!("          inst_id: {}", region.get_instrument_id());
    }
}
