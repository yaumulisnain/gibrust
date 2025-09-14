use __CRATE_IDENT__::prelude::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();
    println!("{}", openapi.to_json().unwrap());
}