use anyhow::Result;
use clap::Parser;

use ld_backup::build_args;
use ld_backup::cli::{BackupConfig, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config: BackupConfig = confy::load("ld-backup", None)?;

    for backup in &config.backups {
        if !std::fs::exists(&backup.target_folder)? {
            return Err(anyhow::anyhow!("Target folder does not exist"));
        }

        let args = build_args(backup, cli.dry_run);

        let output = std::process::Command::new("rsync")
            .args(&args)
            .output()
            .unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}
