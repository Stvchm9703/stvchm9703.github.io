use std::ffi::OsString;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the input file
    #[arg(short = 'i', long)]
    pub import_path: OsString,

    /// Path to the output file
    #[arg(short = 'o', long)]
    pub export_path: OsString,

    /// Number of times to greet
    #[arg(short, long)]
    pub skip_copy: bool,

    #[arg(short, long)]
    pub collections: Option<Vec<String>>,

    #[arg(short, long)]
    pub ts_export: bool,
}
