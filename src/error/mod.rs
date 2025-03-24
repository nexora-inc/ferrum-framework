use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SerializableSqlxError {
  pub message: String,
}

#[derive(Debug)]
pub enum Error {
  DatabaseConnection(SerializableSqlxError),
  DatabaseQuery(SerializableSqlxError),
  DatabaseRowMapping(SerializableSqlxError),
}

impl From<sqlx::Error> for Error {
  fn from(error: sqlx::Error) -> Self {
    let serializable_error = SerializableSqlxError {
      message: error.to_string()
    };

    match error {
      sqlx::Error::PoolTimedOut | sqlx::Error::Configuration(_) => {
        Error::DatabaseConnection(serializable_error)
      },
      sqlx::Error::RowNotFound | sqlx::Error::ColumnIndexOutOfBounds { .. } => {
        Error::DatabaseQuery(serializable_error)
      }, sqlx::Error::ColumnDecode { .. } | sqlx::Error::TypeNotFound { .. } => {
        Error::DatabaseRowMapping(serializable_error)
      }, _ => {
        println!("{:?}", error);
        Error::DatabaseConnection(serializable_error)
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_pool_timeout() {
    // arrange
    let sqlx_error = sqlx::Error::PoolTimedOut;
    let sqlx_error_string = sqlx_error.to_string();

    // act
    let error = Error::from(sqlx_error);

    // assert
    if let Error::DatabaseConnection(db_connection_error) = error {
      assert!(matches!(db_connection_error, SerializableSqlxError { .. }));
      assert_eq!(db_connection_error.message, sqlx_error_string);
    } else {
      panic!("Expected DatabaseConnection error");
    }
  }

  #[test]
  fn test_from_configuration_error() {
    // arrange
    let sqlx_error = sqlx::Error::Configuration(
      "test config error".to_string().into()
    );
    let sqlx_error_string = sqlx_error.to_string();

    // act
    let error = Error::from(sqlx_error);

    // assert
    if let Error::DatabaseConnection(error) = error {
      assert!(matches!(error, SerializableSqlxError { .. }));
      assert_eq!(error.message, sqlx_error_string);
    } else {
      panic!("Expected DatabaseConnection error");
    }
  }
}
