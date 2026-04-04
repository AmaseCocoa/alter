use crate::config;

pub fn list_credentials(profile: String) {
    // Get profile info to verify it exists
    let profile_info = match config::get_profile_from_slug(profile.clone()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load profile '{}': {}", profile, e);
            return;
        }
    };

    // Get credential hosts
    let hosts = match config::get_credential_hosts(profile) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to get credential hosts: {}", e);
            return;
        }
    };

    println!(
        "Credentials for profile '{}' ({}):",
        profile_info.slug, profile_info.email
    );
    println!();

    if hosts.is_empty() {
        println!("  No credentials stored");
    } else {
        for host in hosts {
            println!("  • {} (oauth)", host);
        }
    }
}
