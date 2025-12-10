use gix_config::{file::SectionMut, parse::section::ValueName};
use gix_object::bstr::BString;

use crate::{config, git};

fn set_section_value(
    section: &mut SectionMut,
    key_str: &str,
    value_str: &str,
) {
    let key = match ValueName::try_from(key_str.to_string()) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Internal error: Failed to create ValueName '{}': {}", key_str, e);
            return;
        }
    };
    
    let value: BString = value_str.into();
    section.set(key.into(), value.as_ref());
}

pub fn use_profile(profile: Option<String>, local: bool) {
    match git::GitConfig::load(local) {
        Ok(mut gitconfig) => {
            if let Some(profile) = profile {
                match config::get_profile_from_slug(profile) {
                    Ok(profile) => {
                        if let Ok(mut credential_section) =
                            gitconfig.file.section_mut_or_create_new("credential", None)
                        {
                            set_section_value(&mut credential_section, "namespace", &profile.id.to_string());
                        } else {
                            eprintln!("Failed to create credential section");
                        }

                        if let Ok(mut user_section) =
                            gitconfig.file.section_mut_or_create_new("user", None)
                        {
                            set_section_value(&mut user_section, "email", &profile.email);
                            set_section_value(&mut user_section, "name", &profile.username);
                            if let Some(signing_key) = profile.signing_key {
                                set_section_value(&mut user_section, "signingkey", &signing_key);
                            }
                        } else {
                            eprintln!("Failed to create user section");
                        }
                        let _ = gitconfig.save();
                    }
                    Err(e) => {
                        eprintln!("Error: Failed to find or load profile: {}", e);
                    }
                }
            } else {
                if let Ok(mut credential_section) =
                    gitconfig.file.section_mut_or_create_new("credential", None)
                {
                    set_section_value(&mut credential_section, "namespace", "");
                } else {
                    eprintln!("Failed to create credential section");
                }

                if let Ok(mut user_section) =
                    gitconfig.file.section_mut_or_create_new("user", None)
                {
                    set_section_value(&mut user_section, "email", "");
                    set_section_value(&mut user_section, "name", "");
                    set_section_value(&mut user_section, "signingkey", "");
                } else {
                    eprintln!("Failed to create name section");
                }
                let _ = gitconfig.save();
            }
        }
        Err(e) => {
            eprintln!("Failed to load git config: {}", e);
        }
    }
}
