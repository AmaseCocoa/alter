use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::{fs, io};

use dirs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterProfile {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterUser {
    pub username: String,
    pub email: String,
    #[serde(rename = "signingkey")]
    pub signing_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterProfileConfig {
    pub profile: AlterProfile,
    pub user: AlterUser,
}

#[derive(Debug)]
pub struct ProfileInfo {
    pub id: Uuid,
    pub slug: String,
    pub username: String,
    pub email: String,
    pub signing_key: Option<String>,
}

pub fn get_config_dir() -> Result<PathBuf, io::Error> {
    let mut path: PathBuf = match dirs::home_dir() {
        Some(p) => p,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Home directory not found",
            ));
        }
    };
    path.push(".git-profiles");
    Ok(path)
}

pub fn get_profile_from_slug(slug: String) -> Result<ProfileInfo, io::Error> {
    let mut config_path = get_config_dir()?;
    config_path.push(format!("{}.toml", slug));

    if config_path.exists() {
        match read_profile_config(&config_path) {
            Ok(config) => Ok(ProfileInfo {
                slug,
                id: config.profile.id,
                username: config.user.username,
                email: config.user.email,
                signing_key: config.user.signing_key
            }),
            Err(e) => {
                Err(io::Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to load profile: {}", e),
                ))
            }
        }
    } else {
        Err(io::Error::new(
            ErrorKind::InvalidData,
            "Profile not found",
        ))
    }
}

pub fn list_profiles() -> Result<Vec<ProfileInfo>, io::Error> {
    let config_dir = get_config_dir()?;
    let mut profiles = Vec::new();

    if !config_dir.exists() {
        return Ok(profiles);
    }

    for entry in fs::read_dir(config_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            if slug.is_empty() {
                continue;
            }

            match read_profile_config(&path) {
                Ok(config) => {
                    profiles.push(ProfileInfo {
                        slug,
                        id: config.profile.id,
                        username: config.user.username,
                        email: config.user.email,
                        signing_key: config.user.signing_key
                    });
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Could not read profile file {}: {}",
                        path.display(),
                        e
                    );
                    continue;
                }
            }
        }
    }

    Ok(profiles)
}

fn read_profile_config(path: &Path) -> Result<AlterProfileConfig, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    match toml::from_str(&content) {
        Ok(config) => Ok(config),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse user profile: {}", e),
        )),
    }
}
