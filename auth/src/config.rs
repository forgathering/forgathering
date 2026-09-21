use base64::{Engine, engine::general_purpose};
use rand::{Rng, make_rng, rngs::ChaCha20Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        let mut generator: ChaCha20Rng = make_rng();
        let mut bytes = [0u8; 64];

        generator.fill_bytes(&mut bytes);

        Self {
            jwt_secret: general_purpose::STANDARD.encode(bytes),
        }
    }
}
