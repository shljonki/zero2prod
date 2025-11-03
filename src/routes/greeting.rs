use actix_web::{HttpRequest, Responder};

pub async fn greeting(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("hello {name}")
}