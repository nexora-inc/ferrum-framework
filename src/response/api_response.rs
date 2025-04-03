use lambda_http::{http::StatusCode, Request as HttpRequest, Response};
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

pub struct ApiResponse {
  request: HttpRequest,
  allowed_origins: Vec<String>,
}

impl ApiResponse {
  pub fn new(http_request: HttpRequest, allowed_origins: &[String]) -> Self {
    ApiResponse {
      allowed_origins: allowed_origins.to_vec(),
      request: http_request,
    }
  }

  fn json_response<T: Serialize>(&self, data: &T, status_code: &StatusCode) -> Response<String> {
    let mut builder = Response::builder()
    .status(status_code)
    .header("Content-Type", "application/json")
    .header("Access-Control-Allow-Credentials", "true")
    .header("Access-Control-Allow-Headers", "Content-Type")
    .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET,PUT,PATCH,DELETE");

    let origin_str = self.request.headers().get("origin")
      .and_then(|header_value| header_value.to_str().ok());

    match origin_str {
      Some(origin) => {
        if self.allowed_origins.contains(&origin.to_string()) {
          builder = builder.header("Access-Control-Allow-Origin", origin);
        } else if self.allowed_origins.is_empty() {
          builder = builder.header("Access-Control-Allow-Origin", "*");
        }
      }, None => builder = builder.header("Access-Control-Allow-Origin", "*"),
    };

    builder.body(serde_json::to_string(&data).unwrap_or_default())
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
  use lambda_http::{
    http::{Request as TestRequest, StatusCode},
    Body as TestBody
  };
  use serde::Deserialize;

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct SampleData {
    message: String,
    value: i32,
  }

  fn create_test_request(origin: Option<&str>) -> TestRequest<TestBody> {
    let mut builder = TestRequest::builder();
    if let Some(o) = origin {
      builder = builder.header("Origin", o);
    }
    builder
      .method("GET")
      .uri("http://example.com")
      .body(TestBody::Empty)
      .unwrap()
  }

  #[test]
  fn test_success_with_allowed_origin() {
    let allowed = &["https://example.com".to_string()];
    let request = create_test_request(Some("https://example.com"));
    let api_response = ApiResponse::new(request, allowed);
    let data = SampleData {
      message: "Success!".to_string(),
      value: 200,
    };
    let response = api_response.success(&data);
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
      response.headers().get("Access-Control-Allow-Origin").unwrap(),
      "https://example.com"
    );
  }

  #[test]
  fn test_success_with_disallowed_origin() {
    let allowed = &["https://allowed.com".to_string()];
    let request = create_test_request(Some("https://example.com"));
    let api_response = ApiResponse::new(request, allowed);
    let data = SampleData {
      message: "Success!".to_string(),
      value: 200,
    };
    let response = api_response.success(&data);
    assert!(response.headers().get("Access-Control-Allow-Origin").is_none());
  }
}
