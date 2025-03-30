use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
  pub id: Uuid,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: String,
  pub email: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_auth_user() {
      let id = Uuid::new_v4();
      let first_name = "John".to_string();
      let middle_name = Some("Middle".to_string());
      let last_name = "Doe".to_string();
      let email = "john.doe@example.com".to_string();

      let user = AuthUser {
        id, first_name, middle_name, last_name, email
      };

      assert_eq!(user.id, id);
      assert_eq!(user.first_name, "John");
      assert_eq!(user.middle_name, Some("Middle".to_string()));
      assert_eq!(user.last_name, "Doe");
      assert_eq!(user.email, "john.doe@example.com");
  }

  #[test]
  fn can_create_auth_user_without_middle_name() {
      let id = Uuid::new_v4();
      let first_name = "Jane".to_string();
      let middle_name = None;
      let last_name = "Smith".to_string();
      let email = "jane.smith@example.com".to_string();

      let user = AuthUser {
          id, first_name, middle_name, last_name, email,
      };

      assert_eq!(user.id, id);
      assert_eq!(user.first_name, "Jane");
      assert_eq!(user.middle_name, None);
      assert_eq!(user.last_name, "Smith");
      assert_eq!(user.email, "jane.smith@example.com");
  }
}
