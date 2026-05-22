# Backup script

## Configuration

Create a `default-config.toml` file in `~/.config/ld-backup` with the following structure:

```toml
backups = [
    {
        source_folder = "/home/bobby/my-dir",
        target_folder = "/run/media/bobby/my-hdd/backup",
        exclude_folders = [".git", "node_modules", "vendor"]
    },
    {
        source_folder = "/home/bobby/my-other-dir/personal",
        target_folder = "/run/media/bobby/my-personal-backup",
        exclude_folders = ["build", "dist"]
    }
]
```
