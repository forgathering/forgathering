use actix_web::{
    HttpRequest, HttpResponse, ResponseError, body::BoxBody, dev::HttpServiceFactory, get,
    http::StatusCode,
};
use thiserror::Error;
use utoipa_actix_web::OpenApiFactory;

pub fn mount_api_v0() -> impl HttpServiceFactory + OpenApiFactory + 'static {
    utoipa_actix_web::scope("/v0").service(test_service)
}

#[derive(Debug, Error)]
enum TestError {}

impl ResponseError for TestError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::Ok().finish()
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[utoipa::path(
    get,
    path = "/test",
    params(),
    responses(
        (status = 200, description = "Test completed successfully", body = String),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/test")]
pub async fn test_service(_: HttpRequest) -> Result<String, TestError> {
    Ok("hiiii".to_string())
}
