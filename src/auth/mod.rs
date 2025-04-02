use lambda_http::http::HeaderMap;

use crate::{
  error::SerializableError,
  types::{auth::AuthUser, utils::jwt_util::AuthClaims},
  utils::jwt_util::IJwtUtil,
  Error,
};

pub trait IAuth {
  fn authenticate(&mut self, headers: &HeaderMap) -> Result<(), Error>;
}

pub struct Auth {
  jwt_util: Box<dyn IJwtUtil>,
  claims: Option<AuthClaims>,
  auth_scheme: &'static str,
}

impl Auth {
  pub fn new(jwt_util: Box<dyn IJwtUtil>) -> Self {
    Self { jwt_util, claims: None, auth_scheme: "Watashiwasta " }
  }

  pub fn user(&self) -> Option<AuthUser> {
    match self.claims.as_ref() {
      None => None,
      Some(claims) => Some(claims.user_details.clone()),
    }
  }
}

impl IAuth for Auth {
  fn authenticate(&mut self, headers: &HeaderMap) -> Result<(), Error> {
    let auth_header_value = headers
      .get("Authorization")
      .ok_or_else(|| Error::Unauthorized(SerializableError {
        message: "Missing Authorization header".to_string()
      }))?;

    let auth_header_value_str = auth_header_value.to_str().map_err(|_| {
      Error::Unauthorized(SerializableError {
        message: "Missing Authorization header".to_string()
      })
    })?;

    if !auth_header_value_str.starts_with(self.auth_scheme) {
      return Err(Error::Unauthorized(SerializableError {
        message: "Missing Authorization header".to_string()
      }));
    }

    let token = &auth_header_value_str[self.auth_scheme.len()..];

    self.claims = Some(self.jwt_util.extract_claims(token)?);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Utc;
  use lambda_http::http::HeaderValue;
  use uuid::Uuid;

  use crate::types::utils::jwt_util::TokenType;

  #[cfg(any(test, feature = "mocks"))]
  use crate::utils::jwt_util::MockIJwtUtil;

  #[test]
  fn test_authenticate_success() {
    // arrange
    let mut jwt_util = MockIJwtUtil::new();
    let mut headers = HeaderMap::new();
    let auth_scheme = "Watashiwasta ";
    let raw_token = "valid_token";
    let access_token_value = format!("{}{}", auth_scheme, raw_token);
    let access_token: HeaderValue = access_token_value.parse().unwrap();
    let user_id = Uuid::new_v4();

    headers.insert("Authorization", access_token.clone());

    // act
    jwt_util.expect_extract_claims()
      .with(mockall::predicate::eq(raw_token.to_string())) // Pass the string part of the token
      .times(1)
      .returning(move |_| {
        Ok(AuthClaims {
          subject: user_id.to_string(),
          expires_in: Utc::now().timestamp() as usize,
          user_details: AuthUser {
            id: user_id,
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Doe".to_string(),
            email: "johndoe@example.com".to_string(),
          }, token_type: TokenType::AccessToken,
        })
      });

    // assert
    let mut auth = Auth::new(Box::new(jwt_util));
    assert!(auth.authenticate(&headers).is_ok());
    assert_eq!(auth.user().unwrap().id, user_id);
  }

  #[test]
  fn test_authenticate_missing_header() {
    // arrange
    let mock_jwt_util = MockIJwtUtil::new();
    let mut auth = Auth::new(Box::new(mock_jwt_util));
    let headers = HeaderMap::new();

    // act
    let authenticate_result = auth.authenticate(&headers);

    // assert
    assert!(authenticate_result.is_err());
    match authenticate_result.unwrap_err() {
      Error::Unauthorized(error) => assert_eq!(
        error.message, "Missing Authorization header"
      ), _ => panic!("Unexpected error type"),
    }
  }

  #[test]
  fn test_authenticate_invalid_scheme() {
    // arrange
    let jwt_util = MockIJwtUtil::new();
    let mut headers = HeaderMap::new();
    let auth_scheme = "Invalid ";
    let raw_token = "valid_token";
    let access_token_value = format!("{}{}", auth_scheme, raw_token);
    let access_token: HeaderValue = access_token_value.parse().unwrap();

    headers.insert("Authorization", access_token.clone());

    // act
    let mut auth = Auth::new(Box::new(jwt_util));
    let test_did_fail = auth.authenticate(&headers).is_err();

    // assert
    assert!(test_did_fail);
    assert_eq!(auth.user(), None);
  }

  #[test]
  fn test_authenticate_jwt_extraction_error() {
    // arrange
    let mut mock_jwt_util = MockIJwtUtil::new();
    let mut headers = HeaderMap::new();
    let access_token = "invalid_token";
    headers.insert("Authorization", format!("Watashiwasta {}", access_token)
      .parse()
      .unwrap());

    // act
    mock_jwt_util.expect_extract_claims()
      .with(mockall::predicate::eq(access_token))
      .times(1)
      .returning(|_| {
        Err(Error::JwtTokenInvalid(SerializableError {
          message: "InvalidToken".to_string()
         }))
      });

    let mut auth = Auth::new(Box::new(mock_jwt_util));
    let authenticate_result = auth.authenticate(&headers);

    assert!(authenticate_result.is_err());
    match authenticate_result.unwrap_err() {
      Error::JwtTokenInvalid(_) => assert!(true),
      _ => panic!("Unexpected error type"),
    }
  }
}
