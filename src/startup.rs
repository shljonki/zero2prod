use crate::routes::*;
use actix_web::{App, HttpServer, Route, dev::Server, guard, web, middleware::Logger};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let pg_connection_data = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route(
                "/subscriptions",
                web::post()
                    .guard(guard::Header(
                        "Content-Type",
                        "application/x-www-form-urlencoded",
                    ))
                    .to(subscribe),
            )
            .route("/", Route::new().guard(guard::Get()).to(greeting))
            .route("/{name}", web::get().to(greeting))
            .app_data(pg_connection_data.clone())
    })
    .listen(listener)
    .unwrap()
    .run();
    Ok(server)
}
