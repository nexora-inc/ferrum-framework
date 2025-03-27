pub trait IJwtUtil {
  fn generate_access_token(&self) -> String;
  fn generate_refresh_token(&self) -> String;
}

pub struct JwtUtil;

impl JwtUtil {
  pub fn new() -> Self { Self {} }
}

impl IJwtUtil for JwtUtil {
  fn generate_access_token(&self) -> String {
    "".to_string()
  }
  fn generate_refresh_token(&self) -> String {
    "".to_string()
  }
}
