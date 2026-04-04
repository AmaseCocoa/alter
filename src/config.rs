use std::collections::HashMap;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::{fs, io};

use dirs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Profile-related types (existing functionality)
// ============================================================================

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
pub struct CredentialsMetadata {
    pub hosts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterProfileConfig {
    pub profile: AlterProfile,
    pub user: AlterUser,
    pub credentials: Option<CredentialsMetadata>,
}

#[derive(Debug)]
pub struct ProfileInfo {
    pub id: Uuid,
    pub slug: String,
    pub username: String,
    pub email: String,
    pub signing_key: Option<String>,
}

// ============================================================================
// OAuth Provider types (new functionality)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "gitlab")]
    GitLab,
    #[serde(rename = "gitea")]
    Gitea,
    #[serde(rename = "generic")]
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    pub host: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterConfig {
    pub oauth_providers: HashMap<String, OAuthProvider>,
}

// ============================================================================
// Directory and file management
// ============================================================================

pub fn get_alter_dir() -> Result<PathBuf, io::Error> {
    let mut path: PathBuf = match dirs::home_dir() {
        Some(p) => p,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Home directory not found",
            ));
        }
    };

    path.push(".alter");
    if !path.exists() {
        fs::create_dir(&path)?;
    }

    Ok(path)
}

pub fn get_config_dir() -> Result<PathBuf, io::Error> {
    let alter_dir = get_alter_dir()?;
    let profiles_dir = alter_dir.join("profiles");

    if !profiles_dir.exists() {
        fs::create_dir_all(&profiles_dir)?;
    }

    Ok(profiles_dir)
}

fn get_config_file() -> Result<PathBuf, io::Error> {
    let alter_dir = get_alter_dir()?;
    Ok(alter_dir.join("config.toml"))
}

// ============================================================================
// OAuth Provider configuration
// ============================================================================

pub fn generate_default_config() -> AlterConfig {
    let mut providers = HashMap::new();

    providers.insert(
        "github".to_string(),
        OAuthProvider {
            provider_type: ProviderType::GitHub,
            host: "github.com".to_string(),
            client_id: "Ov23liM4hSM09bDhwmOG".to_string(),
            client_secret: Some("eccb1988f224f0ef54bd78f6b4691f9e854f3449".to_string()),
            auth_endpoint: None,
            token_endpoint: None,
        },
    );

    AlterConfig {
        oauth_providers: providers,
    }
}

pub fn load_config() -> Result<AlterConfig, Box<dyn std::error::Error>> {
    let config_path = get_config_file()?;

    if config_path.exists() {
        let mut file = fs::File::open(&config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        match toml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("Warning: Failed to parse config.toml: {}", e);
                eprintln!("Using default configuration");
                Ok(generate_default_config())
            }
        }
    } else {
        // Generate default config and save it
        let default_config = generate_default_config();
        save_default_config(&default_config)?;
        Ok(default_config)
    }
}

fn save_default_config(_config: &AlterConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_file()?;

    let toml_content = r#"# Alter OAuth Configuration
# This file configures OAuth providers for Git authentication

# GitHub SaaS
[oauth_providers.github]
type = "github"
host = "github.com"
client_id = "Ov23liM4hSM09bDhwmOG"
client_secret = "eccb1988f224f0ef54bd78f6b4691f9e854f3449"

# GitHub Enterprise (uncomment and customize):
# [oauth_providers.ghe]
# type = "github"
# host = "git.company.com"
# client_id = "YOUR_GHE_CLIENT_ID"
# client_secret = "YOUR_GHE_CLIENT_SECRET"

# GitLab SaaS (uncomment and customize):
# [oauth_providers.gitlab]
# type = "gitlab"
# host = "gitlab.com"
# client_id = "YOUR_GITLAB_CLIENT_ID"
# client_secret = "YOUR_GITLAB_CLIENT_SECRET"

# GitLab Self-hosted (uncomment and customize):
# [oauth_providers.gitlab_internal]
# type = "gitlab"
# host = "git.internal.com"
# client_id = "YOUR_GITLAB_ID"
# client_secret = "YOUR_GITLAB_SECRET"

# Gitea (uncomment and customize):
# [oauth_providers.gitea]
# type = "gitea"
# host = "gitea.example.com"
# client_id = "YOUR_GITEA_CLIENT_ID"
# client_secret = "YOUR_GITEA_CLIENT_SECRET"

# Generic OAuth server (uncomment and customize):
# [oauth_providers.custom]
# type = "generic"
# host = "custom.server.com"
# auth_endpoint = "https://custom.server.com/oauth/authorize"
# token_endpoint = "https://custom.server.com/oauth/token"
# client_id = "YOUR_CUSTOM_CLIENT_ID"
# client_secret = "YOUR_CUSTOM_CLIENT_SECRET"
"#;

    fs::write(&config_path, toml_content)?;
    Ok(())
}

pub fn migrate_old_profiles() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

    let old_dir = home_dir.join(".git-profiles");
    let alter_dir = home_dir.join(".alter");
    let new_dir = alter_dir.join("profiles");

    // Check BEFORE creating any directories
    if old_dir.exists() && !new_dir.exists() {
        eprintln!("Migrating profiles from ~/.git-profiles to ~/.alter/profiles...");

        // Create .alter directory if it doesn't exist
        if !alter_dir.exists() {
            fs::create_dir_all(&alter_dir)?;
        }

        // Rename the old profiles directory to the new location
        fs::rename(&old_dir, &new_dir)?;
        eprintln!("✓ Migration complete");
    }

    Ok(())
}

pub fn get_provider_for_host(
    host: &str,
) -> Result<Option<OAuthProvider>, Box<dyn std::error::Error>> {
    let config = load_config()?;

    // Try to find a provider that matches this host exactly
    Ok(config
        .oauth_providers
        .values()
        .find(|p| p.host == host)
        .cloned())
}

fn endpoint_for_provider_type(provider_type: &ProviderType) -> (String, String) {
    match provider_type {
        ProviderType::GitHub => (
            "https://{host}/login/oauth/authorize".to_string(),
            "https://{host}/login/oauth/access_token".to_string(),
        ),
        ProviderType::GitLab => (
            "https://{host}/oauth/authorize".to_string(),
            "https://{host}/oauth/token".to_string(),
        ),
        ProviderType::Gitea => (
            "https://{host}/login/oauth/authorize".to_string(),
            "https://{host}/login/oauth/access_token".to_string(),
        ),
        ProviderType::Generic => {
            // Generic provider must use auth_endpoint and token_endpoint from config
            ("".to_string(), "".to_string())
        }
    }
}

// ============================================================================
// Profile management (existing functionality, preserved)
// ============================================================================

/// Get the actual OAuth endpoint URLs for a provider
pub fn get_auth_token_endpoints(provider: &OAuthProvider) -> (String, String) {
    // If explicit endpoints are provided, use them
    if let (Some(auth), Some(token)) = (&provider.auth_endpoint, &provider.token_endpoint) {
        return (auth.clone(), token.clone());
    }

    // Otherwise, generate from provider type and host
    let (auth_template, token_template) = endpoint_for_provider_type(&provider.provider_type);

    // Replace {host} placeholder with actual host
    let auth_endpoint = auth_template.replace("{host}", &provider.host);
    let token_endpoint = token_template.replace("{host}", &provider.host);

    (auth_endpoint, token_endpoint)
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
                        signing_key: config.user.signing_key,
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
                signing_key: config.user.signing_key,
            }),
            Err(e) => Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("Failed to load profile: {}", e),
            )),
        }
    } else {
        Err(io::Error::new(ErrorKind::InvalidData, "Profile not found"))
    }
}

use crate::git;

#[allow(dead_code)]
pub fn get_current_profile() -> Result<Option<ProfileInfo>, Box<dyn std::error::Error>> {
    let local_config = git::GitConfig::load(true);
    let mut current_id = None;

    if let Ok(config) = local_config {
        if let Ok(section) = config.file.section("credential", None) {
            if let Some(namespace) = section.value("namespace") {
                current_id = Some(namespace.to_string());
            }
        }
    }

    if current_id.is_none() {
        if let Ok(config) = git::GitConfig::load(false) {
            if let Ok(section) = config.file.section("credential", None) {
                if let Some(namespace) = section.value("namespace") {
                    current_id = Some(namespace.to_string());
                }
            }
        }
    }

    if let Some(id_str) = current_id {
        if id_str.is_empty() {
            return Ok(None);
        }

        let profiles = list_profiles()?;
        for profile in profiles {
            if profile.id.to_string() == id_str {
                return Ok(Some(profile));
            }
        }
    }

    Ok(None)
}

pub fn add_host_to_credentials(slug: String, host: String) -> Result<(), io::Error> {
    let mut config_path = get_config_dir()?;
    config_path.push(format!("{}.toml", slug));

    if config_path.exists() {
        let mut config = read_profile_config(&config_path)?;
        let mut metadata = config
            .credentials
            .unwrap_or_else(|| CredentialsMetadata { hosts: Vec::new() });

        if !metadata.hosts.contains(&host) {
            metadata.hosts.push(host);
        }
        config.credentials = Some(metadata);

        let toml_string = match toml::to_string(&config) {
            Ok(s) => s,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to serialize profile: {}", e),
                ));
            }
        };

        fs::write(config_path, toml_string)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Profile not found"))
    }
}

pub fn remove_host_from_credentials(slug: String, host: String) -> Result<(), io::Error> {
    let mut config_path = get_config_dir()?;
    config_path.push(format!("{}.toml", slug));

    if config_path.exists() {
        let mut config = read_profile_config(&config_path)?;
        if let Some(mut metadata) = config.credentials {
            metadata.hosts.retain(|h| h != &host);
            config.credentials = if metadata.hosts.is_empty() {
                None
            } else {
                Some(metadata)
            };

            let toml_string = match toml::to_string(&config) {
                Ok(s) => s,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to serialize profile: {}", e),
                    ));
                }
            };

            fs::write(config_path, toml_string)?;
        }
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Profile not found"))
    }
}

pub fn get_credential_hosts(slug: String) -> Result<Vec<String>, io::Error> {
    let mut config_path = get_config_dir()?;
    config_path.push(format!("{}.toml", slug));

    if config_path.exists() {
        let config = read_profile_config(&config_path)?;
        Ok(config.credentials.map(|c| c.hosts).unwrap_or_default())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Profile not found"))
    }
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
