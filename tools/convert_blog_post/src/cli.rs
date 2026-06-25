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

    /// Skip copying asset files to export dir and static assets dir
    #[arg(short, long)]
    pub skip_copy: bool,

    #[arg(short, long)]
    pub collections: Option<Vec<String>>,

    #[arg(short, long)]
    pub ts_export: bool,

    /// Additional static assets directory to copy files into (default: "static/blog/assets").
    /// Set to an empty string to disable the second copy.
    /// Assets are written to <static_assets_path>/files/<filename>.
    #[arg(long, default_value = "static/blog/assets")]
    pub static_assets_path: OsString,
}
