use crate::{config::list_profiles as get_all_profiles, git};

fn add_pad(c: &String, pad: usize) -> String {
    let slug_len = c.chars().count();
    let padding_width = pad.saturating_sub(slug_len);
    format!("{1:<0$}", padding_width, "")
}

pub fn list_profiles() {
    let gitconfig_global = git::GitConfig::load(false).ok();
    let gitconfig_local = git::GitConfig::load(true).ok();

    let global_id = gitconfig_global
        .as_ref()
        .and_then(|config| config.file.string("credential.namespace").map(|s| s.to_string()));

    let local_id = gitconfig_local
        .as_ref()
        .and_then(|config| config.file.string("credential.namespace").map(|s| s.to_string()));

    let active_id = local_id.as_ref().or(global_id.as_ref());

    match get_all_profiles() {
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
