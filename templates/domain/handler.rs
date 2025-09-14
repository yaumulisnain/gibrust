use axum::{routing::get, Json, Router};
use serde_json::json;
use utoipa::ToSchema;

use crate::app::route::ApiDoc;

#[utoipa::path(get, path = "/__DOMAIN_NAME__/ping", tag = "__DOMAIN_NAME__")]
async fn ping() -> Json<serde_json::Value> {
    Json(json!({"ok": true}))
}

pub fn register___DOMAIN_NAME___routes(router: Router) -> Router {
    router.route("/__DOMAIN_NAME__/ping", get(ping))
}

