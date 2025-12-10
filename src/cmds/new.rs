use std::fs;

use dialoguer::{Confirm, Input};
use uuid::Uuid;

use crate::config;

pub fn new_profile() {
    let slug = Input::<String>::new()
        .with_prompt("slug")
        .allow_empty(false)
        .interact()
        .expect("Failed to read slug. This field is required.");
    let username = Input::<String>::new()
        .with_prompt("username")
        .allow_empty(false)
        .interact()
        .expect("Failed to read username. This field is required.");

    let email = Input::<String>::new()
        .with_prompt("Email Address")
        .allow_empty(false)
        .interact()
        .expect("Failed to read email. This field is required.");

    let gpg_key_result = Input::<String>::new()
        .with_prompt("Enter gpg signing key if you needed")
        .allow_empty(true)
        .interact()
        .expect("Failed to read gpg signing key.");

    let signing_key_option = if gpg_key_result.is_empty() {
        None
    } else {
        Some(gpg_key_result)
    };

    let profile = config::AlterProfileConfig {
        profile: config::AlterProfile { id: Uuid::new_v4() },
        user: config::AlterUser {
            username: username,
            email: email,
            signing_key: signing_key_option,
        },
    };
    
    let mut conf_dir = config::get_config_dir().unwrap();
    conf_dir.push(format!("{}.toml", slug));

    match toml::to_string_pretty(&profile) {
        Ok(profile_toml) => {
            println!("{}", profile_toml);
            let confirm = Confirm::new()
                .with_prompt("Continue?")
                .default(false)
                .interact();
            match confirm {
                Ok(c) => {
                    if c {
                        if conf_dir.exists() {
                            let confirm = Confirm::new()
                                .with_prompt("This slug's profile already exists! Overwrite it?")
                                .default(false)
                                .interact();
                            if confirm.unwrap() {
                                fs::write(conf_dir, profile_toml).expect("Failed to write to file");
                                println!("Profile are written.");
                            }
                        } else {
                            fs::write(conf_dir, profile_toml).expect("Failed to write to file");
                            println!("Profile are written.");
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Unknown Error Occured!!: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize profile!: {}", e);
        }
    }
}
