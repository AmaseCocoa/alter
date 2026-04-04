use crate::config;
use crate::keyring;
use std::io::{self, Write};

pub fn revoke_credential(profile: String, host: Option<String>) {
    // Get profile info
    let profile_info = match config::get_profile_from_slug(profile.clone()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load profile '{}': {}", profile, e);
            return;
        }
    };

    // Get credential hosts
    let hosts = match config::get_credential_hosts(profile.clone()) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to get credential hosts: {}", e);
            return;
        }
    };

    if hosts.is_empty() {
        println!("No credentials stored for profile '{}'", profile);
        return;
    }

    // Determine which host to revoke
    let target_host = if let Some(h) = host {
        h
    } else {
        if hosts.len() == 1 {
            hosts[0].clone()
        } else {
            println!("Available credentials:");
            for (i, h) in hosts.iter().enumerate() {
                println!("  {}. {}", i + 1, h);
            }
            print!("Select host to revoke (number): ");
            io::stdout().flush().ok();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Ok(idx) = input.trim().parse::<usize>() {
                        if idx > 0 && idx <= hosts.len() {
                            hosts[idx - 1].clone()
                        } else {
                            eprintln!("Invalid selection");
                            return;
                        }
                    } else {
                        eprintln!("Invalid input");
                        return;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read input: {}", e);
                    return;
                }
            }
        }
    };

    // Delete from keyring
    match keyring::delete_credential(&profile_info.id.to_string(), &target_host) {
        Ok(_) => {
            println!("✓ Credential removed from keyring");
        }
        Err(e) => {
            eprintln!("Failed to delete credential from keyring: {}", e);
            return;
        }
    }

    // Update profile metadata
    match config::remove_host_from_credentials(profile, target_host.clone()) {
        Ok(_) => {
            println!("✓ Profile updated");
        }
        Err(e) => {
            eprintln!("Failed to update profile: {}", e);
            return;
        }
    }

    println!("✓ Credential for {} revoked", target_host);
}
