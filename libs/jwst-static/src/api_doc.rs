use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use std::{env, sync::Arc};
use utoipa_swagger_ui::{serve, Config, Url};

async fn serve_swagger_ui(
    tail: Option<Path<String>>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    match serve(&tail.map(|p| p.to_string()).unwrap_or("".into()), state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

pub fn with_api_doc_v2(
    router: Router,
    mut openapi: utoipa_v2::openapi::OpenApi,
    title: &'static str,
) -> Router {
    let config = Url::new(title, "/api/jwst.json");
    let config = Config::new(vec![config]);
    openapi.info.license = Some(utoipa_v2::openapi::License::new(env!("CARGO_PKG_LICENSE")));
    router
        .route("/jwst.json", get(move || async { Json(openapi) }))
        .route("/docs/", get(serve_swagger_ui))
        .route("/docs/*tail", get(serve_swagger_ui))
        .layer(Extension(Arc::new(config)))
}

pub fn with_api_doc_v3(
    router: Router,
    mut openapi: utoipa_v3::openapi::OpenApi,
    name: &'static str,
) -> Router {
    if cfg!(debug_assertions) || std::env::var("JWST_DEV").is_ok() {
        let config = Url::from(format!("/api/{name}.json"));
        let config = Config::new(vec![config]);
        openapi.info.license = Some(utoipa_v3::openapi::License::new(env!("CARGO_PKG_LICENSE")));
        router
            .route(
                &format!("/{name}.json"),
                get(move || async { Json(openapi) }),
            )
            .route("/docs/", get(serve_swagger_ui))
            .route("/docs/*tail", get(serve_swagger_ui))
            .layer(Extension(Arc::new(config)))
    } else {
        router
    }
}
