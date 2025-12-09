use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, startup, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "trace".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read config");
    let pg_connection = PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Couldn't connect to database");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("unable to provide port for tcp listener");
    println!("port is: {}", listener.local_addr().unwrap().port());

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    startup::run(listener, pg_connection)?.await
}
