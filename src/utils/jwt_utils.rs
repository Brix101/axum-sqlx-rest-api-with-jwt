use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::config::AppConfig;

use super::errors::{CustomError, CustomResult};
// use mockall::automock;

/// A security service for handling JWT authentication.
pub type DynTokenService = Arc<dyn TokenService + Send + Sync>;

// #[automock]
pub trait TokenService {
    fn new_token(&self, user_id: i64, email: &str) -> CustomResult<String>;
    fn get_user_id_from_token(&self, token: String) -> CustomResult<i64>;
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: i64,
    exp: usize,
}

pub struct JwtService {
    config: Arc<AppConfig>,
}

impl JwtService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl TokenService for JwtService {
    fn new_token(&self, user_id: i64, email: &str) -> CustomResult<String> {
        let from_now = Duration::from_secs(3600);
        let expired_future_time = SystemTime::now().add(from_now);
        let exp = OffsetDateTime::from(expired_future_time);

        let claims = Claims {
            sub: String::from(email),
            exp: exp.unix_timestamp() as usize,
            user_id,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.token_secret.as_bytes()),
        )
        .map_err(|err| CustomError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }

    fn get_user_id_from_token(&self, token: String) -> CustomResult<i64> {
        let decoded_token = decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_secret(self.config.token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| CustomError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims.user_id)
    }
}