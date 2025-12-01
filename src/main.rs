use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, startup};
use tracing_subscriber::{Registry, layer::SubscriberExt, filter::{EnvFilter, LevelFilter}};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    //trace debug info warn error
    
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let filter = EnvFilter::from_default_env()
        .add_directive(LevelFilter::INFO.into());

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(filter);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let configuration = configuration::get_configuration().expect("Failed to read config");
    let pg_connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Couldn't connect to database");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("unable to provide port for tcp listener");
    println!("port is: {}", listener.local_addr().unwrap().port());

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    startup::run(listener, pg_connection)?.await
}
