use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read config");
    let pg_connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Couldn't connect to database");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("unable to provide port for tcp listener");
    println!("port is: {}", listener.local_addr().unwrap().port());
    startup::run(listener, pg_connection)?.await
}
