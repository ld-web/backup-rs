use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Don't execute backup command (dry-run)
    #[arg(short, long)]
    pub dry_run: bool,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackupConfig {
    pub backups: Vec<Config>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub source_folder: PathBuf,
    pub target_folder: PathBuf,
    pub exclude_folders: Vec<PathBuf>,
}
