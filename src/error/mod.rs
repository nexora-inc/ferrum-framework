use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SerializableError {
  pub message: String,
}

#[derive(Debug, Serialize)]
pub enum Error {
  DatabaseConnection(SerializableError),
  DatabaseQuery(SerializableError),
  DatabaseRowNotFound(SerializableError),
  DatabaseRowMapping(SerializableError),
  JWTGenerate(SerializableError),
  Unauthorized(SerializableError),
  ToStr(SerializableError),
}

impl From<sqlx::Error> for Error {
  fn from(error: sqlx::Error) -> Self {
    let serializable_error = SerializableError {
      message: error.to_string()
    };

    match error {
      sqlx::Error::PoolTimedOut | sqlx::Error::Configuration(_) => {
        Error::DatabaseConnection(serializable_error)
      },
      sqlx::Error::RowNotFound => {
        Error::DatabaseRowNotFound(serializable_error)
      }, sqlx::Error::ColumnIndexOutOfBounds { .. } => {
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

impl From<jsonwebtoken::errors::Error> for Error {
  fn from(error: jsonwebtoken::errors::Error) -> Self {
    let serializable_error = SerializableError {
      message: error.to_string()
    };

    Error::JWTGenerate(serializable_error)
  }
}

impl From<lambda_http::http::header::ToStrError> for Error {
  fn from(error: lambda_http::http::header::ToStrError) -> Self {
    Self::ToStr(SerializableError {
      message: error.to_string()
    })
  }
}

#[cfg(test)]
mod tests {
  use lambda_http::http::{header::ToStrError, HeaderValue};

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
      assert!(matches!(db_connection_error, SerializableError { .. }));
      assert_eq!(db_connection_error.message, sqlx_error_string);
    } else {
      panic!("Expected DatabaseConnection error");
    }
  }

  #[test]
  fn test_from_row_not_found() {
    // arrange
    let sqlx_error = sqlx::Error::RowNotFound;
    let sqlx_error_string = sqlx_error.to_string();

    // act
    let error = Error::from(sqlx_error);

    // assert
    if let Error::DatabaseRowNotFound(not_found_error) = error {
      assert!(matches!(not_found_error, SerializableError { .. }));
      assert_eq!(not_found_error.message, sqlx_error_string);
    } else {
      panic!("Expected DatabaseRowNotFound error");
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
      assert!(matches!(error, SerializableError { .. }));
      assert_eq!(error.message, sqlx_error_string);
    } else {
      panic!("Expected DatabaseConnection error");
    }
  }

  #[test]
  fn test_from_jwt_generate_error() {
    // arrange
    let jwt_error = jsonwebtoken::errors::Error::from(
      jsonwebtoken::errors::ErrorKind::InvalidSignature
    );
    let jwt_error_string = jwt_error.to_string();

    // act
    let error = Error::from(jwt_error);

    // assert
    if let Error::JWTGenerate(jwt_generate_error) = error {
      assert!(matches!(jwt_generate_error, SerializableError { .. }));
      assert_eq!(jwt_generate_error.message, jwt_error_string);
    } else {
      panic!("Expected JWTGenerate error");
    }
  }

  #[test]
  fn test_from_lambda_http_to_str_error() {
    // arrange
    let invalid_bytes = b"\xf0\x9f\xa6\xad\xed\xa0\x80";
        let header_value = HeaderValue::from_bytes(invalid_bytes).unwrap();
    let to_str_result: Result<&str, ToStrError> = header_value.to_str();
    let to_str_error = to_str_result.unwrap_err();

    // act
    let error: Error = to_str_error.into();

    // assert
    match error {
      Error::ToStr(serializable_error) => {
        assert_eq!(serializable_error.message, "failed to convert header to a str");
      }, _ => panic!("Expected Error::ToStr variant, but got {:?}", error),
    }
  }
}
