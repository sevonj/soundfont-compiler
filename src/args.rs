use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Project manifest path
    #[arg(short, long, default_value_t = String::from("./SoundFont.toml"))]
    pub path: String,

    /// Print in detail
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Validate
    #[arg(short, long, default_value_t = false)]
    pub check: bool,
}
