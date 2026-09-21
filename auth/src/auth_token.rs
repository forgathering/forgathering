use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Debug)]
pub struct AuthToken(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenClaims {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub expiration: u64,
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum AuthTokenError {
    #[error("Failed to encode auth token: {0}")]
    EncodingError(jsonwebtoken::errors::Error),

    #[error("Failed to decode auth token: {0}")]
    DecodingError(jsonwebtoken::errors::Error),
}

impl AuthToken {
    pub(crate) fn get_data(
        &self,
        secret: &[u8],
    ) -> Result<TokenData<AuthTokenClaims>, AuthTokenError> {
        decode::<AuthTokenClaims>(
            &self.0,
            &DecodingKey::from_secret(secret),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(AuthTokenError::DecodingError)
    }

    pub(crate) fn create(
        session_id: Uuid,
        user_id: Uuid,
        secret: &[u8],
    ) -> Result<AuthToken, AuthTokenError> {
        let expiration = (SystemTime::now() + Duration::from_days(3))
            .duration_since(UNIX_EPOCH)
            .expect("Future expiration duration should be after UNIX_EPOCH")
            .as_secs();

        let token = encode(
            &Header::default(),
            &AuthTokenClaims {
                user_id,
                session_id,
                expiration,
            },
            &EncodingKey::from_secret(secret),
        )
        .map_err(AuthTokenError::EncodingError)?;

        Ok(AuthToken(token))
    }

    pub(crate) fn expired(&self, secret: &[u8]) -> Result<bool, AuthTokenError> {
        Ok(SystemTime::now()
            < (UNIX_EPOCH + Duration::from_secs(self.get_data(secret)?.claims.expiration)))
    }
}

impl From<String> for AuthToken {
    fn from(value: String) -> Self {
        AuthToken(value)
    }
}
