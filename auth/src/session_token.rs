use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SessionToken(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenClaims {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub expiration: u64,
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum SessionTokenError {
    #[error("Failed to encode session token: {0}")]
    EncodingError(jsonwebtoken::errors::Error),

    #[error("Failed to decode session token: {0}")]
    DecodingError(jsonwebtoken::errors::Error),
}

impl SessionToken {
    pub(crate) fn get_data(
        &self,
        secret: &[u8],
    ) -> Result<TokenData<SessionTokenClaims>, SessionTokenError> {
        decode::<SessionTokenClaims>(
            &self.0,
            &DecodingKey::from_secret(secret),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(SessionTokenError::DecodingError)
    }

    pub(crate) fn create(
        session_id: Uuid,
        user_id: Uuid,
        secret: &[u8],
    ) -> Result<SessionToken, SessionTokenError> {
        let expiration = (SystemTime::now() + Duration::from_days(3))
            .duration_since(UNIX_EPOCH)
            .expect("Future expiration duration should be after UNIX_EPOCH")
            .as_secs();

        let token = encode(
            &Header::default(),
            &SessionTokenClaims {
                user_id,
                session_id,
                expiration,
            },
            &EncodingKey::from_secret(secret),
        )
        .map_err(SessionTokenError::EncodingError)?;

        Ok(SessionToken(token))
    }

    pub(crate) fn expired(&self, secret: &[u8]) -> Result<bool, SessionTokenError> {
        Ok(SystemTime::now()
            < (UNIX_EPOCH + Duration::from_secs(self.get_data(secret)?.claims.expiration)))
    }
}

impl From<String> for SessionToken {
    fn from(value: String) -> Self {
        SessionToken(value)
    }
}
