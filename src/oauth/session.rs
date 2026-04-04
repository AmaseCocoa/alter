use std::sync::OnceLock;

use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use tiny_http::{Response, Server};
use url::Url;

use crate::oauth::pkce::PKCESecret;

#[derive(Serialize)]
struct OAuth2TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

#[derive(Deserialize, Debug)]
pub struct OAuth2TokenResponse {
    pub access_token: String,
}

pub struct OAuth2TemporaryServer {
    #[allow(dead_code)]
    pub port: u16,
    pub redirect_uri: String,
}

pub struct OAuth2Session {
    client_id: String,
    client_secret: String,
    server: OnceLock<Server>,
    host: String,
    auth_endpoint: Option<String>,
    token_endpoint: Option<String>,
}

impl OAuth2Session {
    pub fn new(client_id: String, client_secret: String, host: String) -> Self {
        Self {
            client_id,
            client_secret,
            server: OnceLock::new(),
            host,
            auth_endpoint: None,
            token_endpoint: None,
        }
    }

    /// Create a new session with custom OAuth endpoints
    pub fn with_endpoints(
        client_id: String,
        client_secret: String,
        host: String,
        auth_endpoint: Option<String>,
        token_endpoint: Option<String>,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            server: OnceLock::new(),
            host,
            auth_endpoint,
            token_endpoint,
        }
    }

    pub fn create_server(&mut self) -> OAuth2TemporaryServer {
        self.server
            .get_or_init(|| Server::http("127.0.0.1:0").expect("Failed to start server"));

        let server = self.server.get().unwrap();

        let local_addr = server.server_addr();
        let port = local_addr.to_ip().unwrap().port();

        let redirect_uri = format!("http://127.0.0.1:{}", &port);

        OAuth2TemporaryServer { port, redirect_uri }
    }

    pub fn wait_code(&self) -> Option<String> {
        if let Some(server) = self.server.get() {
            if let Some(request) = server.incoming_requests().next() {
                let url = Url::parse(&format!("http://127.0.0.1{}", request.url())).unwrap();
                let code = url
                    .query_pairs()
                    .find(|(key, _)| key == "code")
                    .map(|(_, value)| value.into_owned());

                let response =
                    Response::from_string("Authentication complete! You can close this tab.");
                let _ = request.respond(response);

                return code;
            }
        }
        None
    }

    pub fn auth_url(&self, server: &OAuth2TemporaryServer, pkce: &PKCESecret) -> String {
        // Use custom auth endpoint if provided, otherwise default to GitHub style
        let auth_endpoint = if let Some(ref endpoint) = self.auth_endpoint {
            endpoint.clone()
        } else {
            format!("{}/login/oauth/authorize", self.host)
        };

        format!(
            "{}?client_id={}&redirect_uri={}&scope=repo&code_challenge={}&code_challenge_method=S256",
            auth_endpoint,
            self.client_id,
            urlencoding::encode(&server.redirect_uri),
            pkce.code_challenge
        )
    }

    pub fn get_token(
        &self,
        code: String,
        server: &OAuth2TemporaryServer,
        pkce: &PKCESecret,
    ) -> Result<OAuth2TokenResponse, Box<dyn std::error::Error>> {
        let client = Client::new();
        let body = OAuth2TokenRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code,
            code_verifier: pkce.code_verifier.clone(),
            redirect_uri: server.redirect_uri.clone(),
        };
        let mut header = HeaderMap::new();
        header.insert(ACCEPT, HeaderValue::from_static("application/json"));

        // Use custom token endpoint if provided, otherwise default to GitHub style
        let token_endpoint = if let Some(ref endpoint) = self.token_endpoint {
            endpoint.clone()
        } else {
            format!("{}/login/oauth/access_token", self.host)
        };

        let res = client
            .post(token_endpoint)
            .headers(header)
            .json(&body)
            .send();

        let response = res?.error_for_status()?;
        let token_res: OAuth2TokenResponse = response.json::<OAuth2TokenResponse>()?;

        Ok(token_res)
    }
}
