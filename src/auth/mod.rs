use lambda_http::http::HeaderMap;

use crate::{
  error::SerializableError,
  types::{auth::AuthUser, utils::jwt_util::AuthClaims},
  utils::jwt_util::IJwtUtil,
  Error
};

pub trait IAuth {
  fn authenticate(&mut self, headers: &HeaderMap) -> Result<(), Error>;
}

pub struct Auth {
  jwt_util: Box<dyn IJwtUtil>,
  claims: Option<AuthClaims>,
}

impl Auth {
  pub fn new(jwt_util: Box<dyn IJwtUtil>) -> Self {
    Self { jwt_util, claims: None }
  }

  pub fn user(&self) -> AuthUser {
    self.claims.as_ref().unwrap().user_details.clone()
  }
}

impl IAuth for Auth {
  fn authenticate(&mut self, headers: &HeaderMap) -> Result<(), Error> {
    let auth_header_value = headers.get("Authorization")
      .ok_or(Error::Unauthorized(SerializableError {
        message: "Unauthorized.".to_string()
      }))?;

    if auth_header_value.is_empty() {
      return Err(Error::Unauthorized(SerializableError {
        message: "Unauthorized.".to_string()
      }))
    }

    let auth_header_value_string = auth_header_value.to_str()?;

    if !auth_header_value_string.starts_with("Watashiwasta ") {
      return Err(Error::Unauthorized(SerializableError {
        message: "Unauthorized.".to_string()
      }))
    }

    let token = &auth_header_value_string["Watashiwasta ".len()..];

    self.claims = Some(self.jwt_util.extract_claims(token)?);

    Ok(())
  }
}
