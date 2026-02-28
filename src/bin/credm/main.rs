mod oauth;

use oauth::session::OAuth2Session;
use oauth::pkce::PKCESecret;

fn main() {
    let client_id = "Ov23liM4hSM09bDhwmOG";
    let client_secret = "eccb1988f224f0ef54bd78f6b4691f9e854f3449";
    let host = "https://github.com";
    let mut session = OAuth2Session::new(client_id.to_string(), client_secret.to_string(), host.to_string());
    let server = &session.create_server();
    let pkce = PKCESecret::new();

    println!("割り当てられたポート: {}", server.port);
    println!(
        "ブラウザで開いてください: {}",
        session.auth_url(server, &pkce)
    );

    if let Some(code) = session.wait_code() {
        let token = session.get_token(code, server, &pkce);
        println!("{}", token.unwrap().access_token);
    }
}
