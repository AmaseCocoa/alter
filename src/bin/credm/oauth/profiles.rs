use std::{collections::HashMap, sync::LazyLock};

struct OAuth2Profile {
    client_id: String,
    client_secret: Option<String>,
    scope: Vec<String>
}

static OAUTH2_PROFILES: LazyLock<HashMap<&'static str, OAuth2Profile>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("github.com", OAuth2Profile {
        
        client_id: "Ov23liM4hSM09bDhwmOG".to_string(),
        client_secret: Some("eccb1988f224f0ef54bd78f6b4691f9e854f3449".to_string()),
        scope: ["repo".to_string(), "gist".to_string(), "workflow".to_string()].to_vec()
    });
    m
});