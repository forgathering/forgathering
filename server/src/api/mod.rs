use actix_web::{
    HttpResponse, Responder,
    dev::HttpServiceFactory,
    get,
    http::header::ContentType,
    web::{self},
};
use utoipa::openapi::OpenApi;
use utoipa_actix_web::OpenApiFactory;

use crate::api::v0::mount_api_v0;

pub mod v0;

#[utoipa::path(responses((status = OK, body = str)))]
#[get("/openapi.json")]
async fn api_docs_json(spec: web::Data<OpenApi>) -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::json()).body(
        serde_json::to_string_pretty(&spec)
            .unwrap_or_else(|_| "Error converting to JSON".to_string()),
    )
}

pub fn mount_api() -> impl HttpServiceFactory + OpenApiFactory + 'static {
    utoipa_actix_web::scope("/api").service(mount_api_v0())
}
