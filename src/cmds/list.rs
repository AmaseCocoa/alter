use crate::{config, git, keyring};

fn add_pad(c: &str, pad: usize) -> String {
    let len = c.chars().count();
    let padding_width = pad.saturating_sub(len);
    format!("{1:<0$}", padding_width, "")
}

fn show_profile_status(profile_id_str: &str, host: &str) -> String {
    match keyring::get_credential(profile_id_str, host) {
        Ok(_) => "logged in".to_string(),
        Err(_) => "not logged in".to_string(),
    }
}

pub fn show_current_profile() {
    let gitconfig_global = git::GitConfig::load(false).ok();
    let gitconfig_local = git::GitConfig::load(true).ok();

    let global_id = gitconfig_global.as_ref().and_then(|config| {
        config
            .file
            .string("credential.namespace")
            .map(|s| s.to_string())
    });

    let local_id = gitconfig_local.as_ref().and_then(|config| {
        config
            .file
            .string("credential.namespace")
            .map(|s| s.to_string())
    });

    let current_id = local_id.as_ref().or(global_id.as_ref());

    let Some(current_id) = current_id else {
        println!("No active profile.");
        return;
    };

    match config::list_profiles() {
        Ok(profiles) => {
            if let Some(profile) = profiles.iter().find(|p| p.id.to_string() == *current_id) {
                let host_list = match config::get_credential_hosts(profile.slug.clone()) {
                    Ok(hosts) => hosts,
                    Err(_) => Vec::new(),
                };

                println!("Current profile:");
                println!("  Slug: {}", profile.slug);
                println!("  ID: {}", profile.id);
                println!("  Username: {}", profile.username);
                println!("  Email: {}", profile.email);
                println!(
                    "  Signing key: {}",
                    profile.signing_key.as_deref().unwrap_or("(none)")
                );

                if host_list.is_empty() {
                    println!("  Credentials: none");
                } else {
                    println!("  Credentials:");
                    for host in host_list {
                        println!(
                            "    - {} ({})",
                            host,
                            show_profile_status(&profile.id.to_string(), &host)
                        );
                    }
                }

                if local_id.as_ref() == Some(current_id) {
                    println!("  Scope: local");
                } else {
                    println!("  Scope: global");
                }
            } else {
                println!("Current profile id: {}", current_id);
                println!("Profile details not found.");
            }
        }
        Err(e) => {
            eprintln!("Failed to load profiles: {}", e);
        }
    }
}

pub fn list_profiles() {
    let gitconfig_global = git::GitConfig::load(false).ok();
    let gitconfig_local = git::GitConfig::load(true).ok();

    let global_id = gitconfig_global.as_ref().and_then(|config| {
        config
            .file
            .string("credential.namespace")
            .map(|s| s.to_string())
    });

    let local_id = gitconfig_local.as_ref().and_then(|config| {
        config
            .file
            .string("credential.namespace")
            .map(|s| s.to_string())
    });

    let active_id = local_id.as_ref().or(global_id.as_ref());

    match config::list_profiles() {
        Ok(profiles) => {
            let max_slug_len = profiles.iter().map(|p| p.slug.len()).max().unwrap_or(0);
            let max_username_len = profiles.iter().map(|p| p.username.len()).max().unwrap_or(0);

            for profile in profiles {
                let profile_id_str = profile.id.to_string();
                let mut tags = Vec::new();

                if local_id.as_ref() == Some(&profile_id_str) {
                    tags.push("[LOCAL]");
                }
                if global_id.as_ref() == Some(&profile_id_str) {
                    tags.push("[GLOBAL]");
                }

                let prefix = if active_id == Some(&profile_id_str) {
                    "*"
                } else {
                    " "
                };

                println!(
                    "{} {}{}({}){}<{}>{}{}",
                    prefix,
                    profile.slug,
                    add_pad(&profile.slug, max_slug_len + 2),
                    profile.username,
                    add_pad(&format!("({})", profile.username), max_username_len + 4),
                    profile.email,
                    add_pad(&format!("<{}>", profile.email), 25),
                    tags.join(" ")
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to load profiles: {}", e);
        }
    }
}
