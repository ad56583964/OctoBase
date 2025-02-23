mod api_doc;
mod files;
mod pages;

use axum::{
    body::{boxed, BoxBody, Full},
    http::{header::CONTENT_TYPE, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::EmbeddedFile;

type StaticFileFetcher = fn(&str) -> Option<EmbeddedFile>;

pub use api_doc::{with_api_doc_v2, with_api_doc_v3};
pub use files::fetch_static_response;
pub use rust_embed::{self, RustEmbed};
