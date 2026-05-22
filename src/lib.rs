pub mod cli;

use crate::cli::Config;

pub fn build_args(backup: &Config, dry_run: bool) -> Vec<String> {
    // rsync options:
    // -h: output numbers in a human-readable format
    // -v: verbose output
    // -r: recurse into directories
    // -P: show progress during transfer and keep partially transferred files
    // -t: preserve modification times
    let mut args = vec!["-hvrPt".to_string()];

    if dry_run {
        args.push("--dry-run".to_string());
    }

    args.push(backup.source_folder.to_string_lossy().into_owned());
    args.push(backup.target_folder.to_string_lossy().into_owned());

    for exclude in &backup.exclude_folders {
        args.push("--exclude".to_string());
        args.push(format!("{}/", exclude.to_string_lossy()));
    }

    args
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_build_args() {
        let backup = Config {
            source_folder: PathBuf::from("/home/bobby/my-dir"),
            target_folder: PathBuf::from("/run/media/bobby/my-hdd/backup"),
            exclude_folders: vec![
                PathBuf::from(".git"),
                PathBuf::from("node_modules"),
                PathBuf::from("vendor"),
            ],
        };

        let args = build_args(&backup, false);
        assert_eq!(
            args,
            vec![
                "-hvrPt",
                "/home/bobby/my-dir",
                "/run/media/bobby/my-hdd/backup",
                "--exclude",
                ".git/",
                "--exclude",
                "node_modules/",
                "--exclude",
                "vendor/"
            ]
        );
    }

    #[test]
    fn test_build_args_with_dry_run() {
        let backup = Config {
            source_folder: PathBuf::from("/home/bobby/my-dir"),
            target_folder: PathBuf::from("/run/media/bobby/my-hdd/backup"),
            exclude_folders: vec![
                PathBuf::from(".git"),
                PathBuf::from("node_modules"),
                PathBuf::from("vendor"),
            ],
        };

        let args = build_args(&backup, true);
        assert_eq!(
            args,
            vec![
                "-hvrPt",
                "--dry-run",
                "/home/bobby/my-dir",
                "/run/media/bobby/my-hdd/backup",
                "--exclude",
                ".git/",
                "--exclude",
                "node_modules/",
                "--exclude",
                "vendor/"
            ]
        );
    }

    #[test]
    fn test_without_exclude_folders() {
        let backup = Config {
            source_folder: PathBuf::from("/home/bobby/my-dir"),
            target_folder: PathBuf::from("/run/media/bobby/my-hdd/backup"),
            exclude_folders: vec![],
        };

        let args = build_args(&backup, false);
        assert_eq!(
            args,
            vec![
                "-hvrPt",
                "/home/bobby/my-dir",
                "/run/media/bobby/my-hdd/backup"
            ]
        );
    }
}
