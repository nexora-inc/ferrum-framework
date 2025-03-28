use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{types::utils::jwt_util::AuthClaims, Error};

pub trait IJwtUtil {
  fn generate_token(&self, claims: &AuthClaims) -> Result<String, Error>;
}

pub struct JwtUtil {
  encoding_key: EncodingKey,
}

impl JwtUtil {
  pub fn new(app_key: &str) -> Self {
    let encoding_key = EncodingKey::from_secret(app_key.as_bytes());

    Self { encoding_key }
  }
}

impl IJwtUtil for JwtUtil {
  fn generate_token(&self, claims: &AuthClaims) -> Result<String, Error> {
    Ok(encode(&Header::default(), claims, &self.encoding_key)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{Duration, Utc};
  use jsonwebtoken::{decode, DecodingKey, Validation};
  use uuid::Uuid;

  use crate::types::{auth::AuthUser, utils::jwt_util::TokenType};

  #[test]
  fn test_generate_token() {
    // arrange
    let app_key = "test-app-key";
    let jwt_util = JwtUtil::new(app_key);
    let user_id = Uuid::new_v4();
    let expires_in_seconds = Utc::now() + Duration::hours(1);
    let auth_user = AuthUser {
      id: user_id,
      first_name: "Test".to_string(),
      middle_name: None,
      last_name: "User".to_string(),
      email: "test.user@example.com".to_string(),
    };
    let claims = AuthClaims {
      subject: user_id.to_string(),
      expires_in: expires_in_seconds.timestamp() as usize,
      user_details: auth_user,
      token_type: TokenType::AccessToken,
    };

    // act
    let token_result = jwt_util.generate_token(&claims);

    // assert
    assert!(token_result.is_ok());
    let token = token_result.unwrap();
    assert!(!token.is_empty());
    let decoding_key = DecodingKey::from_secret(app_key.as_bytes());
    let validation = Validation::default();
    let decoded_result = decode::<AuthClaims>(&token, &decoding_key, &validation);
    assert!(decoded_result.is_ok());
    let decoded_claims = decoded_result.unwrap().claims;
    assert_eq!(decoded_claims.subject, claims.subject);
    assert_eq!(decoded_claims.expires_in, claims.expires_in);
    assert_eq!(decoded_claims.user_details.id, claims.user_details.id);
    assert_eq!(decoded_claims.token_type, claims.token_type);
  }
}
