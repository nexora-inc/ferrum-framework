use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{types::utils::jwt_util::{AuthClaims, TokenType}, Error};

#[cfg(any(test, feature = "mocks"))]
use mockall::{automock, predicate::*};
#[async_trait::async_trait]
#[cfg_attr(any(test, feature = "mocks"), automock)]
pub trait IJwtUtil: Send + Sync {
  fn generate_token(&self, subject: &str, token_type: &TokenType) -> Result<String, Error>;
  fn extract_claims(&self, token: &str) -> Result<AuthClaims, Error>;
}

#[derive(Clone)]
pub struct JwtUtil {
  encoding_key: EncodingKey,
  decoding_key: DecodingKey,
}

impl JwtUtil {
  pub fn new(app_key: &str) -> Self {
    let encoding_key = EncodingKey::from_secret(app_key.as_bytes());
    let decoding_key = DecodingKey::from_secret(app_key.as_bytes());

    Self { encoding_key, decoding_key }
  }
}

impl IJwtUtil for JwtUtil {
  fn generate_token(&self, subject: &str, token_type: &TokenType) -> Result<String, Error> {
    let expiry_date = Utc::now().timestamp();
    let claims = AuthClaims {
        subject: subject.to_string(),
        expires_in: expiry_date,
        token_type: token_type.clone(),
    };

    Ok(encode(&Header::default(), &claims, &self.encoding_key)?)
  }

  fn extract_claims(&self, token: &str) -> Result<AuthClaims, Error> {
    Ok(decode::<AuthClaims>(
      token, &self.decoding_key, &Validation::default()
    )?.claims)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use jsonwebtoken::{decode, DecodingKey, Validation};
  use uuid::Uuid;

  use crate::types::utils::jwt_util::TokenType;

  #[test]
  fn test_generate_token() {
    // arrange
    let app_key = "test-app-key";
    let jwt_util = JwtUtil::new(app_key);
    let user_id = Uuid::new_v4();
    let subject = user_id.to_string();
    let token_type = TokenType::AccessToken;

    // act
    let token_result = jwt_util.generate_token(&subject, &token_type);

    // assert
    assert!(token_result.is_ok());
    let token = token_result.unwrap();
    assert!(!token.is_empty());
    let decoding_key = DecodingKey::from_secret(app_key.as_bytes());
    let validation = Validation::default();
    let decoded_result = decode::<AuthClaims>(&token, &decoding_key, &validation);
    assert!(decoded_result.is_ok());
    let decoded_claims = decoded_result.unwrap().claims;
    assert_eq!(decoded_claims.subject, subject);
    assert_eq!(decoded_claims.token_type, token_type);
  }

  #[test]
  fn test_extract_claims_success() {
    // arrange
    let jwt_util = JwtUtil::new("some_key");
    let subject = Uuid::new_v4().to_string();
    let token_type = TokenType::AccessToken;
    let token = jwt_util.generate_token(&subject, &token_type)
      .ok().unwrap();

    // act
    let extract_claims_result = jwt_util.extract_claims(&token);

    // assert
    assert!(extract_claims_result.is_ok());
    let extract_claims = extract_claims_result.unwrap();
    assert_eq!(extract_claims.subject, subject);
    assert_eq!(extract_claims.token_type, token_type);
  }

  #[test]
  fn test_extract_claims_invalid_token() {
    // arrange
    let jwt_util = JwtUtil::new("some_key");

    // act
    let extract_claims_result = jwt_util.extract_claims("invalid token");

    // assert
    assert!(extract_claims_result.is_err());
    if let Err(Error::JwtTokenInvalid(error)) = extract_claims_result {
      assert_eq!(error.message, "InvalidToken");
    } else {
      panic!("Unexpected error type");
    }
  }
}
