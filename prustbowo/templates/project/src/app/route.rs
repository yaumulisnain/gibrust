use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// <prustbowo:imports>

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(schemas()),
    tags((name = "api", description = "API endpoints"))
)]
pub struct ApiDoc;

pub fn app_router(router: Router) -> Router {
    let mut router = router;
    // <prustbowo:routes>
    router = router.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));
    router
}

