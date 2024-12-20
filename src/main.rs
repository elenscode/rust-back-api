mod errors;
mod handler;
mod swagger;

use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::swagger::ApiDoc;

fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/v1")
        .service(handler::index)
        .service(handler::warning);
    conf.service(scope);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(config).configure(config).service(
            SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
