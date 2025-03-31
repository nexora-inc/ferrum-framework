use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{types::utils::jwt_util::AuthClaims, Error};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait IJwtUtil {
  fn generate_token(&self, claims: &AuthClaims) -> Result<String, Error>;
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
  fn generate_token(&self, claims: &AuthClaims) -> Result<String, Error> {
    Ok(encode(&Header::default(), claims, &self.encoding_key)?)
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

  #[test]
  fn test_extract_claims_success() {
    // arrange
    let jwt_util = JwtUtil::new("some_key");
    let auth_user = AuthUser {
      id: Uuid::new_v4(),
      first_name: "John".to_string(),
      middle_name: None,
      last_name: "John".to_string(),
      email: "John".to_string()
    };
    let claims = AuthClaims {
       subject: auth_user.id.to_string(),
       expires_in: Utc::now().timestamp() as usize,
       user_details: auth_user,
       token_type: TokenType::AccessToken,
    };
    let token = jwt_util.generate_token(&claims)
      .unwrap();

    // act
    let extract_claims_result = jwt_util.extract_claims(&token);

    // assert
    assert!(extract_claims_result.is_ok());
    let extract_claims = extract_claims_result.unwrap();
    assert_eq!(extract_claims, claims);
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
