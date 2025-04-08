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
    ApiResponse {}
  }

  fn json_response<T: Serialize>(&self, data: &T, status_code: &StatusCode) -> Response<String> {
    Response::builder()
      .status(status_code)
      .header("Content-Type", "application/json")
      .header("Access-Control-Allow-Origin", "*")
      .header("Access-Control-Allow-Headers", "Content-Type")
      .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET,PUT,PATCH,DELETE")
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
    use serde_json::{json, Value};

    fn get_response_body(response: &Response<String>) -> Value {
        serde_json::from_str(response.body()).unwrap_or_default()
    }

    fn check_cors_headers(response: &Response<String>) {
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(
            response.headers().get("Access-Control-Allow-Origin").unwrap(),
            "*"
        );
        assert_eq!(
            response.headers().get("Access-Control-Allow-Headers").unwrap(),
            "Content-Type"
        );
        assert_eq!(
            response.headers().get("Access-Control-Allow-Methods").unwrap(),
            "OPTIONS,POST,GET,PUT,PATCH,DELETE"
        );
    }

    #[test]
    fn test_success() {
        let api_response = ApiResponse::new();
        let data = json!({"message": "Success"});
        let response = api_response.success(&data);

        assert_eq!(response.status(), StatusCode::OK);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body, data);
    }

    #[test]
    fn test_success_with_status() {
        let api_response = ApiResponse::new();
        let data = json!({"message": "Custom status"});
        let response = api_response.success_with_status(&data, &StatusCode::ACCEPTED);

        assert_eq!(response.status(), StatusCode::ACCEPTED);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body, data);
    }

    #[test]
    fn test_created() {
        let api_response = ApiResponse::new();
        let data = json!({"id": 1, "name": "New resource"});
        let response = api_response.created(&data);

        assert_eq!(response.status(), StatusCode::CREATED);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body, data);
    }

    #[test]
    fn test_bad_request() {
        let api_response = ApiResponse::new();
        let message = "Invalid input";
        let response = api_response.bad_request(message);

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body["message"], message);
    }

    #[test]
    fn test_unauthorized() {
        let api_response = ApiResponse::new();
        let response = api_response.unauthorized();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body["message"], "Unauthorized.");
    }

    #[test]
    fn test_forbidden() {
        let api_response = ApiResponse::new();
        let message = "Access denied";
        let response = api_response.forbidden(message);

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body["message"], message);
    }

    #[test]
    fn test_not_found() {
        let api_response = ApiResponse::new();
        let data = json!({"resource": "User", "id": 123});
        let response = api_response.not_found(&data);

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body, data);
    }

    #[test]
    fn test_unprocessable_entity() {
        let api_response = ApiResponse::new();
        let data = json!({"errors": {"name": ["is required"]}});
        let response = api_response.unprocessable_entity(&data);

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body, data);
    }

    #[test]
    fn test_server_error() {
        let api_response = ApiResponse::new();
        let data = json!({"message": "Internal server error"});
        let response = api_response.server_error(&data);

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body, data);
    }

    #[test]
    fn test_error_with_status() {
        let api_response = ApiResponse::new();
        let data = json!({"message": "Service unavailable"});
        let response = api_response.error_with_status(&data, &StatusCode::SERVICE_UNAVAILABLE);

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        check_cors_headers(&response);

        let body = get_response_body(&response);
        assert_eq!(body, data);
    }
}
