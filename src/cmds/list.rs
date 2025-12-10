use crate::{config::list_profiles as get_all_profiles, git};

fn add_pad(c: &String, pad: usize) -> String {
    let slug_len = c.chars().count();
    let padding_width = pad.saturating_sub(slug_len);
    format!("{1:<0$}", padding_width, "")
}

pub fn list_profiles() {
    let gitconfig = git::GitConfig::load(false);
    let gitconfig_local = git::GitConfig::load(true);

    let pure_global_id = gitconfig
        .as_ref()
        .ok()
        .and_then(|config| config.file.string("credential.namespace").map(|s| s.to_string()));

    let current_active_id = gitconfig_local
        .as_ref()
        .ok()
        .and_then(|config| config.file.string("credential.namespace").map(|s| s.to_string()));

    match get_all_profiles() {
        Ok(profiles) => {
            for profile in profiles {
                let profile_id = profile.id.to_string();

                let is_current_active = current_active_id
                    .as_ref()
                    .map(|id| id == &profile_id)
                    .unwrap_or(false);

                let location_tag = if is_current_active {
                    let is_local_override = match (&current_active_id, &pure_global_id) {
                        (Some(c_id), Some(g_id)) if c_id != g_id => true,
                        (Some(_), None) => true,
                        _ => false,
                    };

                    if is_local_override {
                        "[LOCAL]"
                    } else {
                        "[GLOBAL]"
                    }
                } else {
                    ""
                };

                let prefix = if is_current_active { "*" } else { " " };

                println!(
                    "{} {}{}({}){}<{}>{}{}",
                    prefix,
                    profile.slug,
                    add_pad(&profile.slug, 15),
                    profile.username,
                    add_pad(&format!("({})", profile.username), 15),
                    profile.email,
                    add_pad(&format!("<{}>", profile.email), 25),
                    location_tag
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to load profiles: {}", e);
        }
    }

    if let Err(e) = gitconfig {
        eprintln!("Failed to load global git config: {}", e);
    }
    if let Err(e) = gitconfig_local {
        eprintln!("Failed to load local git config: {}", e);
    }
}
