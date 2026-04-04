use std::io::{self, Write};

use crate::config;
use crate::keyring;
use crate::oauth::{pkce::PKCESecret, profiles::get_profile_for_host, session::OAuth2Session};

pub fn setup_credentials(profile: String, host: Option<String>) {
    // Get profile info
    let profile_info = match config::get_profile_from_slug(profile.clone()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load profile '{}': {}", profile, e);
            return;
        }
    };

    // Determine host
    let target_host = if let Some(h) = host {
        h
    } else {
        print!("Enter Git host (e.g., github.com): ");
        io::stdout().flush().ok();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(e) => {
                eprintln!("Failed to read host input: {}", e);
                return;
            }
        }
    };

    if target_host.is_empty() {
        eprintln!("Host cannot be empty");
        return;
    }

    // Get OAuth2 profile for host
    let oauth_profile = match get_profile_for_host(&target_host) {
        Some(p) => p,
        None => {
            eprintln!("No OAuth2 profile available for host: {}", target_host);
            eprintln!("Supported hosts: github.com");
            return;
        }
    };

    // Start OAuth2 flow
    println!("Starting OAuth2 authentication for {}...", target_host);

    let mut session = OAuth2Session::new(
        oauth_profile.client_id.to_string(),
        oauth_profile.client_secret.unwrap_or("").to_string(),
        format!("https://{}", target_host),
    );

    let server = session.create_server();
    let pkce = PKCESecret::new();

    println!("Opening browser for authentication...");
    println!(
        "If browser doesn't open, visit: {}",
        session.auth_url(&server, &pkce)
    );

    // Try to open browser
    let _ = open::that(session.auth_url(&server, &pkce));

    if let Some(code) = session.wait_code() {
        match session.get_token(code, &server, &pkce) {
            Ok(token_response) => {
                // Store token in keyring
                let profile_id = profile_info.id.to_string();
                match keyring::set_credential(
                    &profile_id,
                    &target_host,
                    &token_response.access_token,
                ) {
                    Ok(_) => {
                        println!("✓ Token stored in system keyring");
                        eprintln!(
                            "  Debug: service=alter, username={}:{}",
                            profile_id, target_host
                        );

                        // Verify token was actually stored
                        match keyring::get_credential(&profile_id, &target_host) {
                            Ok(_) => {
                                eprintln!(
                                    "  Debug: Verification successful - token is retrievable"
                                );
                            }
                            Err(e) => {
                                eprintln!("  Warning: Token storage verification failed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to store token in keyring: {}", e);
                        eprintln!(
                            "  Trying to store: service=alter, username={}:{}",
                            profile_id, target_host
                        );
                        return;
                    }
                }

                // Update profile metadata
                match config::add_host_to_credentials(profile, target_host.clone()) {
                    Ok(_) => {
                        println!("✓ Profile updated with host: {}", target_host);
                    }
                    Err(e) => {
                        eprintln!("Failed to update profile: {}", e);
                        return;
                    }
                }

                println!("✓ OAuth2 setup complete for {}", target_host);
            }
            Err(e) => {
                eprintln!("Failed to obtain token: {}", e);
            }
        }
    } else {
        eprintln!("Authentication failed or cancelled");
    }
}
