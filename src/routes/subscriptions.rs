use serde::Deserialize;
use actix_web::{web, HttpResponse};

#[derive(Deserialize)]
pub struct FormData {
    name: Option<String>,
    email: Option<String>
}

// isto je što se tiče performansa ocemo li vratit impl Responder ili HttpResponse
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("name {:?}", form.name.as_ref());
    println!("mail {:?}", form.email.as_ref());
    match (form.name.as_ref(), form.email.as_ref()) {
        (Some(_), Some(_)) => HttpResponse::Ok().finish(),
        (Some(_), None) => HttpResponse::BadRequest().body("missing email"),
        (None, Some(_)) => HttpResponse::BadRequest().body("missing name"),
        _ => HttpResponse::BadRequest().body("missing name and email")
    }
}