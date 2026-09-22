#![feature(duration_constructors)]

pub mod config;
pub mod session_token;

use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, password_hash};
use base64::{Engine, engine::general_purpose};
use jsonwebtoken::TokenData;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    config::AuthConfig,
    session_token::{SessionToken, SessionTokenClaims, SessionTokenError},
};

#[derive(Clone)]
pub struct AuthEngine<'a> {
    argon2: Argon2<'a>,

    jwt_secret: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum StartAuthError {
    #[error("Failed to decode base64: {0}")]
    Base64DecodeFailed(base64::DecodeError),

    #[error("Invalid Argon2 Parameters: {0}")]
    InvalidArgonParameters(argon2::Error),
}

#[derive(Error, Debug)]
pub enum PasswordHashError {
    #[error("Failed to hash password: {0}")]
    HashingFailed(password_hash::Error),
}

#[derive(Error, Debug)]
pub enum PasswordValidationError {
    #[error("Failed to parse stored password hash: {0}")]
    ParseFailed(argon2::password_hash::phc::Error),
}

impl AuthEngine<'_> {
    pub fn open(config: AuthConfig) -> Result<Self, StartAuthError> {
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::new(19 * 1024, 2, 1, Some(32))
                .map_err(StartAuthError::InvalidArgonParameters)?,
        );

        Ok(Self {
            argon2,

            jwt_secret: general_purpose::STANDARD
                .decode(config.jwt_secret)
                .map_err(StartAuthError::Base64DecodeFailed)?,
        })
    }

    pub fn hash_password_for_storage(&self, password: &str) -> Result<String, PasswordHashError> {
        Ok(self
            .argon2
            .hash_password(password.as_bytes())
            .map_err(PasswordHashError::HashingFailed)?
            .to_string())
    }

    pub fn validate_password(
        &self,
        password: &str,
        stored_hash: &str,
    ) -> Result<bool, PasswordValidationError> {
        let parsed_hash =
            PasswordHash::new(stored_hash).map_err(PasswordValidationError::ParseFailed)?;

        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn create_auth_token(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<SessionToken, SessionTokenError> {
        SessionToken::create(session_id, user_id, &self.jwt_secret)
    }

    pub fn get_auth_token_data(
        &self,
        token: &SessionToken,
    ) -> Result<TokenData<SessionTokenClaims>, SessionTokenError> {
        token.get_data(&self.jwt_secret)
    }

    pub fn auth_token_expired(&self, token: &SessionToken) -> Result<bool, SessionTokenError> {
        token.expired(&self.jwt_secret)
    }
}
