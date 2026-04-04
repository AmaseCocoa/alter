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
    pub port: u16,
    pub redirect_uri: String,
}

pub struct OAuth2Session {
    client_id: String,
    client_secret: String,
    server: OnceLock<Server>,
    host: String,
}

impl OAuth2Session {
    pub fn new(client_id: String, client_secret: String, host: String) -> Self {
        Self {
            client_id,
            client_secret,
            server: OnceLock::new(),
            host,
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
        format!(
            "{}/login/oauth/authorize?client_id={}&redirect_uri={}&scope=repo&code_challenge={}&code_challenge_method=S256",
            self.host,
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

        let res = client
            .post(format!("{}/login/oauth/access_token", self.host))
            .headers(header)
            .json(&body)
            .send();

        let response = res?.error_for_status()?;
        let token_res: OAuth2TokenResponse = response.json::<OAuth2TokenResponse>()?;

        Ok(token_res)
    }
}
