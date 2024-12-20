use utoipa::{openapi, OpenApi};

#[derive(OpenApi)]
#[openapi(paths(super::handler::index, super::handler::warning,))]
pub struct ApiDoc;
