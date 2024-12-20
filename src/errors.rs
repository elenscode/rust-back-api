use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)] // `Error` 매크로는 `Debug`와 `Display`를 자동으로 구현합니다.
pub enum MyError {
    #[error("custom error message")] // `Display` 구현
    CustomError,
    #[error("io error: {0}")] // `Display` 구현
    IoError(#[from] std::io::Error),
    #[error("default error")] // `Display` 구현
    DefaultError,
}

impl actix_web::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::CustomError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::IoError(_) => StatusCode::GATEWAY_TIMEOUT,
            MyError::DefaultError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self.to_string())
    }
}
