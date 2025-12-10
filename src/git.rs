use std::path::{Path, PathBuf};
use gix_config::{File, Source};
use std::fs::write;

pub struct GitConfig {
    pub file: File<'static>,
    pub path: PathBuf,
}

impl GitConfig {
    pub fn load(repo: bool) -> Result<Self, Box<dyn std::error::Error>> {
        if repo {
            if !Path::new(".git").exists() {
                return Err("Not a git repository.".into());
            }
            let git_dir = PathBuf::from(".git");
            let path = git_dir.join("config");
            let file = File::from_path_no_includes(path.clone(), Source::Local)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(Self { file, path })
        } else {
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| {
                std::env::var("USERPROFILE").unwrap_or_default()
            });
            let path = PathBuf::from(home_dir).join(".gitconfig");
            let file = File::from_path_no_includes(path.clone(), Source::User)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(Self { file, path })
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_string = self.file.to_string();
        write(&self.path, config_string)?;
        Ok(())
    }
}