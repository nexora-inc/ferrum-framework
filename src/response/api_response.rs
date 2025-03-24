use lambda_http::{http::StatusCode, Response};
use serde::Serialize;

pub trait IApiResponse {
  fn success<T: Serialize>(data: T) -> Response<String>;

  fn success_with_status<T: Serialize>(data: T, status_code: StatusCode) -> Response<String>;

  fn created<T: Serialize>(data: T) -> Response<String>;

  fn unprocessable_entity<T: Serialize>(data: T) -> Response<String>;

  fn server_error<T: Serialize>(data: T) -> Response<String>;

  fn error_with_status<T: Serialize>(data: T, status_code: StatusCode) -> Response<String>;
}

pub struct ApiResponse;

impl ApiResponse {
  fn json_response<T: Serialize>(data: T, status_code: StatusCode) -> Response<String> {
    Response::builder()
      .status(status_code)
      .header("Content-Type", "application/json")
      .header("Access-Control-Allow-Origin", "application/json")
      .header("Access-Control-Allow-Headers", "Content-Type")
      .header("Access-Control-Allow-Methods", "OPTIONS,POST")
      .body(serde_json::to_string(&data).unwrap_or_default())
      .unwrap()
  }
}

impl IApiResponse for ApiResponse {
  fn success<T: Serialize>(data: T) -> Response<String> {
    Self::json_response(data, StatusCode::OK)
  }

  fn success_with_status<T: Serialize>(data: T, status_code: StatusCode) -> Response<String> {
    Self::json_response(data, status_code)
  }

  fn created<T: Serialize>(data: T) -> Response<String> {
    Self::json_response(data, StatusCode::CREATED)
  }

  fn unprocessable_entity<T: Serialize>(data: T) -> Response<String> {
    Self::json_response(data, StatusCode::UNPROCESSABLE_ENTITY)
  }

  fn server_error<T: Serialize>(data: T) -> Response<String> {
    Self::json_response(data, StatusCode::INTERNAL_SERVER_ERROR)
  }

  fn error_with_status<T: Serialize>(data: T, status_code: StatusCode) -> Response<String> {
    Self::json_response(data, status_code)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use lambda_http::http::StatusCode;
  use serde::Deserialize;
  use serde_json;

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct SampleData {
    message: String,
    value: i32,
  }

  #[test]
  fn test_success() {
    let data = SampleData {
      message: "Success!".to_string(),
      value: 200,
    };
    let response = ApiResponse::success(&data);
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
      response.headers().get("Content-Type").unwrap(),
      "application/json"
    );
    let body = response.body();
    let deserialized_body: SampleData = serde_json::from_str(body.as_str()).unwrap();
    assert_eq!(deserialized_body, data);
  }

  #[test]
  fn test_success_with_status() {
    let data = SampleData {
      message: "Custom Success!".to_string(),
      value: 201,
    };
    let response = ApiResponse::success_with_status(&data, StatusCode::CREATED);
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
      response.headers().get("Content-Type").unwrap(),
      "application/json"
    );
    let body = response.body();
    let deserialized_body: SampleData = serde_json::from_str(body.as_str()).unwrap();
    assert_eq!(deserialized_body, data);
  }

  #[test]
  fn test_created() {
    let data = SampleData {
      message: "Created!".to_string(),
      value: 201,
    };
    let response = ApiResponse::created(&data);
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
      response.headers().get("Content-Type").unwrap(),
      "application/json"
    );
    let body = response.body();
    let deserialized_body: SampleData = serde_json::from_str(body.as_str()).unwrap();
    assert_eq!(deserialized_body, data);
  }

  #[test]
  fn test_unprocessable_entity() {
    let data = SampleData {
      message: "Validation Error!".to_string(),
      value: 422,
    };
    let response = ApiResponse::unprocessable_entity(&data);
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
      response.headers().get("Content-Type").unwrap(),
      "application/json"
    );
    let body = response.body();
    let deserialized_body: SampleData = serde_json::from_str(body.as_str()).unwrap();
    assert_eq!(deserialized_body, data);
  }

  #[test]
  fn test_server_error() {
    let data = SampleData {
      message: "Server Error!".to_string(),
      value: 500,
    };
    let response = ApiResponse::server_error(&data);
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
      response.headers().get("Content-Type").unwrap(),
      "application/json"
    );
    let body = response.body();
    let deserialized_body: SampleData = serde_json::from_str(body.as_str()).unwrap();
    assert_eq!(deserialized_body, data);
  }

  #[test]
  fn test_error_with_status() {
    let data = SampleData {
      message: "Custom Error!".to_string(),
      value: 400,
    };
    let response = ApiResponse::error_with_status(&data, StatusCode::BAD_REQUEST);
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
      response.headers().get("Content-Type").unwrap(),
      "application/json"
    );
    let body = response.body();
    let deserialized_body: SampleData = serde_json::from_str(body.as_str()).unwrap();
    assert_eq!(deserialized_body, data);
  }
}
