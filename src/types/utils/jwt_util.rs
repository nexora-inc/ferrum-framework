use serde::{Deserialize, Serialize};

use crate::types::auth::AuthUser;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
  AccessToken,
  RefreshToken,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthClaims {
  #[serde(rename = "sub")]
  pub subject: String,
  #[serde(rename = "exp")]
  pub expires_in: usize,
  pub user_details: AuthUser,
  pub token_type: TokenType,
}

#[cfg(test)]
mod tests {
  use super::*;
  use uuid::Uuid;

  use crate::types::auth::AuthUser;

  #[test]
  fn can_create_auth_claims_with_access_token() {
    let user_id = Uuid::new_v4();
    let auth_user = AuthUser {
      id: user_id,
      first_name: "John".to_string(),
      middle_name: None,
      last_name: "Doe".to_string(),
      email: "john.doe@example.com".to_string(),
    };

    let subject = user_id.to_string();
    let expires_in = 3600;
    let token_type = TokenType::AccessToken;

    let claims = AuthClaims {
      subject, expires_in, user_details: auth_user, token_type,
    };

    assert_eq!(claims.subject, user_id.to_string());
    assert_eq!(claims.expires_in, 3600);
    assert_eq!(claims.user_details.id, user_id);
    assert_eq!(claims.user_details.first_name, "John");
    assert_eq!(claims.token_type, TokenType::AccessToken);
  }

  #[test]
  fn can_create_auth_claims_with_refresh_token() {
    let user_id = Uuid::new_v4();
    let auth_user = AuthUser {
      id: user_id,
      first_name: "Jane".to_string(),
      middle_name: Some("Middle".to_string()),
      last_name: "Smith".to_string(),
      email: "jane.smith@example.com".to_string(),
    };

    let subject = user_id.to_string();
    let expires_in = 86400;
    let token_type = TokenType::RefreshToken;

    let claims = AuthClaims {
      subject, expires_in, user_details: auth_user, token_type,
    };

    assert_eq!(claims.subject, user_id.to_string());
    assert_eq!(claims.expires_in, 86400);
    assert_eq!(claims.user_details.id, user_id);
    assert_eq!(claims.user_details.first_name, "Jane");
    assert_eq!(claims.token_type, TokenType::RefreshToken);
  }
}
