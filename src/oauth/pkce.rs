use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use sha2::{Digest, Sha256};

pub struct PKCESecret {
    pub code_verifier: String,
    pub code_challenge: String,
}

impl PKCESecret {
    pub fn new() -> Self {
        let secret = Self::generate_secret(43);
        let hash = Sha256::digest(secret.as_bytes());
        let code_challenge = URL_SAFE_NO_PAD.encode(hash);

        Self {
            code_verifier: secret,
            code_challenge: code_challenge,
        }
    }

    fn generate_secret(length: usize) -> String {
        Alphanumeric.sample_string(&mut rand::rng(), length)
    }
}
