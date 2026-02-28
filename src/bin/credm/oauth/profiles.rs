use std::{collections::HashMap, sync::LazyLock};

pub struct OAuth2Profile {
    client_id: &'static str,
    client_secret: Option<&'static str>,
    scope: &'static [&'static str],
}

/// OAuth2 Profiles for various Git hosts.
/// 
/// SECURITY NOTE:
/// The `client_secret` values are intentionally hardcoded and public.
/// As this is a client-side Open Source (OSS) tool, obfuscating these secrets 
/// provides no genuine security (security by obscurity), as they can be 
/// easily extracted from the compiled binary. 
/// We prioritize a seamless "zero-config" user experience and transparency.
/// Security is primarily maintained via the OAuth2 PKCE flow.
static OAUTH2_PROFILES: LazyLock<HashMap<&'static str, OAuth2Profile>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("github.com", OAuth2Profile {
        client_id: "Ov23liM4hSM09bDhwmOG",
        client_secret: Some(concat!("eccb1988f224f0ef54bd", "78f6b4691f9e854f3449")),
        scope: &["repo", "gist", "workflow"],
    });
    m
});

fn resolve_profile_key(host: &str) -> &str {
    // 1. Static aliases
    match host {
        "gist.github.com" | "api.github.com" => return "github.com",
        _ => {}
    }

    // 2. GHE Cloud and GitHub branding domains
    if host == "github.com" || host.ends_with(".github.com") || host.ends_with(".ghe.com") {
        return "github.com";
    }

    host
}

pub fn get_profile_for_host(host: &str) -> Option<&'static OAuth2Profile> {
    let key = resolve_profile_key(host);
    OAUTH2_PROFILES.get(key)
}