use crate::config;
use crate::git;
use crate::keyring;
use uuid::Uuid;

pub fn debug_credential(profile_id_or_slug: String, host: Option<String>) {
    println!("=== Alter Credential Debug ===\n");

    // Try to resolve profile_id_or_slug to actual profile
    let profile_info = if Uuid::parse_str(&profile_id_or_slug).is_ok() {
        // It's a UUID, find the profile with this ID
        match config::list_profiles() {
            Ok(profiles) => profiles
                .into_iter()
                .find(|p| p.id.to_string() == profile_id_or_slug)
                .ok_or_else(|| format!("Profile with ID {} not found", profile_id_or_slug)),
            Err(e) => Err(format!("Failed to list profiles: {}", e)),
        }
    } else {
        // It's a slug
        config::get_profile_from_slug(profile_id_or_slug.clone())
            .map_err(|e| format!("Failed to load profile: {}", e))
    };

    let profile_info = match profile_info {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("Profile Information:");
    println!("  Slug: {}", profile_info.slug);
    println!("  ID: {}", profile_info.id);
    println!("  Email: {}", profile_info.email);
    println!();

    // Check git config
    println!("Git Config Status:");
    if let Ok(config) = git::GitConfig::load(true) {
        println!("  Local (repo) config:");
        if let Ok(section) = config.file.section("credential", None) {
            if let Some(namespace) = section.value("namespace") {
                println!("    credential.namespace: {}", namespace);
            } else {
                println!("    credential.namespace: (not set)");
            }
            if let Some(helper) = section.value("helper") {
                println!("    credential.helper: {}", helper);
            } else {
                println!("    credential.helper: (not set)");
            }
        } else {
            println!("    credential section not found");
        }
    } else {
        println!("  Local (repo) config: Not in a git repository");
    }

    if let Ok(config) = git::GitConfig::load(false) {
        println!("  Global (~/.gitconfig):");
        if let Ok(section) = config.file.section("credential", None) {
            if let Some(namespace) = section.value("namespace") {
                println!("    credential.namespace: {}", namespace);
            } else {
                println!("    credential.namespace: (not set)");
            }
            if let Some(helper) = section.value("helper") {
                println!("    credential.helper: {}", helper);
            } else {
                println!("    credential.helper: (not set)");
            }
        } else {
            println!("    credential section not found");
        }
    }
    println!();

    // Check stored credentials
    let hosts_to_check = if let Some(h) = host {
        vec![h]
    } else {
        match config::get_credential_hosts(profile_info.slug.clone()) {
            Ok(hosts) => {
                if hosts.is_empty() {
                    println!("No credentials registered in profile metadata");
                    return;
                }
                hosts
            }
            Err(e) => {
                eprintln!("Failed to get credential hosts: {}", e);
                return;
            }
        }
    };

    println!("Credential Storage Check:");
    for h in hosts_to_check {
        println!("  Host: {}", h);
        println!("    Keyring lookup:");
        println!("      Service: alter");
        println!("      Username: {}:{}", profile_info.id, h);

        match keyring::get_credential(&profile_info.id.to_string(), &h) {
            Ok(token) => {
                let masked_token = if token.len() > 10 {
                    format!("{}...{}", &token[..5], &token[token.len() - 5..])
                } else {
                    "***".to_string()
                };
                println!("      Status: ✓ Found (token: {})", masked_token);
            }
            Err(e) => {
                println!("      Status: ✗ Not found");
                println!("      Error: {}", e);
            }
        }
        println!();
    }

    println!("=== End Debug ===");
}
