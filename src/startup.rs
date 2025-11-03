
use actix_web::{dev::Server, guard, web, App, HttpServer, Route};
use std::net::TcpListener;
use crate::routes::*;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().guard(guard::Header("Content-Type", "application/x-www-form-urlencoded")).to(subscribe))
            .route("/", Route::new().guard(guard::Get()).to(greeting))
            .route("/{name}", web::get().to(greeting))
        })
        .listen(listener).unwrap()
        .run();
    Ok(server)
}
