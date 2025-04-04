use lambda_http::{http::StatusCode, Response};
use serde::Serialize;
use serde_json::json;

pub trait IApiResponse {
  fn success<T: Serialize>(&self, data: &T) -> Response<String>;

  fn success_with_status<T: Serialize>(&self, data: &T, status_code: &StatusCode) -> Response<String>;

  fn created<T: Serialize>(&self, data: &T) -> Response<String>;

  fn bad_request(&self, message: &str) -> Response<String>;

  fn unauthorized(&self) -> Response<String>;

  fn forbidden(&self, message: &str) -> Response<String>;

  fn not_found<T: Serialize>(&self, data: &T) -> Response<String>;

  fn unprocessable_entity<T: Serialize>(&self, data: &T) -> Response<String>;

  fn server_error<T: Serialize>(&self, data: &T) -> Response<String>;

  fn error_with_status<T: Serialize>(&self, data: &T, status_code: &StatusCode) -> Response<String>;
}

pub struct ApiResponse;

impl ApiResponse {
  pub fn new() -> Self {
    Self {}
  }

  fn json_response<T: Serialize>(&self, data: &T, status_code: &StatusCode) -> Response<String> {
    Response::builder()
      .status(status_code)
      .header("Content-Type", "application/json")
      .body(serde_json::to_string(&data).unwrap_or_default())
        .unwrap()
  }
}

impl IApiResponse for ApiResponse {
  fn success<T: Serialize>(&self, data: &T) -> Response<String> {
    self.json_response(data, &StatusCode::OK)
  }

  fn success_with_status<T: Serialize>(&self, data: &T, status_code: &StatusCode) -> Response<String> {
    self.json_response(data, status_code)
  }

  fn created<T: Serialize>(&self, data: &T) -> Response<String> {
    self.json_response(data, &StatusCode::CREATED)
  }

  fn bad_request(&self, message: &str) -> Response<String> {
    self.json_response(&json!({
      "message": message
    }), &StatusCode::BAD_REQUEST)
  }

  fn unauthorized(&self) -> Response<String> {
    self.json_response(&json!({
      "message": "Unauthorized."
    }), &StatusCode::UNAUTHORIZED)
  }

  fn forbidden(&self, message: &str) -> Response<String> {
    self.json_response(&json!({
      "message": message,
    }), &StatusCode::FORBIDDEN)
  }

  fn not_found<T: Serialize>(&self, data: &T) -> Response<String> {
    self.json_response(data, &StatusCode::NOT_FOUND)
  }

  fn unprocessable_entity<T: Serialize>(&self, data: &T) -> Response<String> {
    self.json_response(data, &StatusCode::UNPROCESSABLE_ENTITY)
  }

  fn server_error<T: Serialize>(&self, data: &T) -> Response<String> {
    self.json_response(data, &StatusCode::INTERNAL_SERVER_ERROR)
  }

  fn error_with_status<T: Serialize>(&self, data: &T, status_code: &StatusCode) -> Response<String> {
    self.json_response(&data, status_code)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use lambda_http::http::StatusCode;
  use serde::Deserialize;

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
    let api_response = ApiResponse::new();
    let response = api_response.success(&data);
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
    let api_response = ApiResponse::new();
    let response = api_response.success_with_status(&data, &StatusCode::CREATED);
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
    let api_response = ApiResponse::new();
    let response = api_response.created(&data);
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
    let api_response = ApiResponse::new();
    let response = api_response.unprocessable_entity(&data);
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
    let api_response = ApiResponse::new();
    let response = api_response.server_error(&data);
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
    let api_response = ApiResponse::new();
    let response = api_response.error_with_status(&data, &StatusCode::BAD_REQUEST);
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
