pub enum Error {
  DatabaseConnection(sqlx::Error),
  DatabaseQuery(sqlx::Error),
  DatabaseRowMapping(sqlx::Error),
}

impl From<sqlx::Error> for Error {
  fn from(error: sqlx::Error) -> Self {
    match error {
      sqlx::Error::PoolTimedOut | sqlx::Error::Configuration(_) => {
        Error::DatabaseConnection(error)
      },
      sqlx::Error::RowNotFound | sqlx::Error::ColumnIndexOutOfBounds { .. } => {
        Error::DatabaseQuery(error)
      }, sqlx::Error::ColumnDecode { .. } | sqlx::Error::TypeNotFound { .. } => {
        Error::DatabaseRowMapping(error)
      }, _ => {
        println!("{:?}", error);
        Error::DatabaseConnection(error)
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

    // act
    let error = Error::from(sqlx_error);

    // assert
    if let Error::DatabaseConnection(db_connection_error) = error {
      assert!(matches!(db_connection_error, sqlx::Error::PoolTimedOut));
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

    // act
    let error = Error::from(sqlx_error);

    // assert
    if let Error::DatabaseConnection(e) = error {
      assert!(matches!(e, sqlx::Error::Configuration(_)));
    } else {
      panic!("Expected DatabaseConnection error");
    }
  }
}
