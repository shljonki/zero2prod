//! tests/health_check.rs
use actix_web::{HttpResponse, Responder};

// ako koristimo -> impl Responder onda mozemo vratit i HttpResponse::Ok()
// to je HttpResponseBuilder koji implementira Responder trait
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
