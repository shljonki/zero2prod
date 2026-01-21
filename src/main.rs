use sqlx::postgres::PgPoolOptions;
use std::{net::TcpListener, time::Duration};
use zero2prod::{configuration, startup, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // set up logging
    //    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "trace".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read config");

    // connect to database inside container
    let pg_connection = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        .max_connections(5)
        .connect_lazy_with(configuration.database.with_db());

    // set up app address which are allowed to send requests to our app
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    println!("{}", address);
    let listener = TcpListener::bind(address).expect("unable to provide port for tcp listener");

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    startup::run(listener, pg_connection)?.await
}
