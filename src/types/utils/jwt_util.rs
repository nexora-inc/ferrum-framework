use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
  AccessToken,
  RefreshToken,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthClaims {
  #[serde(rename = "sub")]
  pub subject: String,
  #[serde(rename = "exp")]
  pub expires_in: i64,
  pub token_type: TokenType,
}

#[cfg(test)]
mod tests {
  use super::*;
  use uuid::Uuid;

  #[test]
  fn can_create_auth_claims_with_access_token() {
    let user_id = Uuid::new_v4();
    let subject = user_id.to_string();
    let expires_in = 3600;
    let token_type = TokenType::AccessToken;

    let claims = AuthClaims {
      subject, expires_in, token_type,
    };

    assert_eq!(claims.subject, user_id.to_string());
    assert_eq!(claims.expires_in, 3600);
    assert_eq!(claims.token_type, TokenType::AccessToken);
  }

  #[test]
  fn can_create_auth_claims_with_refresh_token() {
    let user_id = Uuid::new_v4();
    let subject = user_id.to_string();
    let expires_in = 86400;
    let token_type = TokenType::RefreshToken;

    let claims = AuthClaims {
      subject, expires_in, token_type,
    };

    assert_eq!(claims.subject, user_id.to_string());
    assert_eq!(claims.expires_in, 86400);
    assert_eq!(claims.token_type, TokenType::RefreshToken);
  }
}
