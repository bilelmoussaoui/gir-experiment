use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Copy, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
pub enum Mode {
    Doc,
    Normal,
    Sys,
    NotBound,
}

#[derive(Debug, Parser)]
#[cfg_attr(test, derive(Default))]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Config file path (default: Gir.toml)
    #[arg(short, long, value_name = "CONFIG")]
    config: Option<PathBuf>,
    /// Work mode
    #[arg(value_enum, short, long, value_name = "MODE")]
    mode: Option<Mode>,

    /// Target path
    #[arg(short = 'o', long, value_name = "PATH")]
    target: Option<PathBuf>,
    /// Directories for GIR files
    #[arg(short = 'd', long, value_name = "GIRSPATH")]
    pub(super) girs_directories: Vec<PathBuf>,
    /// Doc target path
    #[arg(short = 'p', long, value_name = "PATH")]
    doc_target: Option<PathBuf>,
    #[arg(short = 'b', long)]
    make_backup: Option<bool>,
    /// Show statistics
    #[arg(short = 's', long)]
    stats: Option<bool>,
    #[arg(long)]
    disable_format: Option<bool>,
    /// Check if the given `.gir` file is valid
    #[arg(long, value_name = "PATH")]
    check_gir_file: Option<PathBuf>,
}

impl Args {
    pub fn mode(&self) -> Option<Mode> {
        self.mode
    }

    pub fn girs_directories(&self) -> &[PathBuf] {
        &self.girs_directories
    }
}
