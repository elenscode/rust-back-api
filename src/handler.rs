use crate::errors::MyError;
use actix_web::{get, HttpResponse};

#[utoipa::path(
    context_path = "/v1",
    tag = "handler",
    responses(
        (status = 200, description = "success response"),
    )
)]
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().json("Hello world!"))
}

#[utoipa::path(
    context_path = "/v1",
    tag = "handler",
    responses(
        (status = 200, description = "success response"),
    )
)]
#[get("/warning")]
async fn warning() -> Result<HttpResponse, MyError> {
    Err(MyError::CustomError)
}
