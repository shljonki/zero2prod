//! tests/health_check.rs
use once_cell::sync::Lazy;
use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{env, net::TcpListener};
use uuid::Uuid;
use zero2prod::{
    configuration::{self, DataBaseSettings},
    startup, telemetry,
};

static TRACING: Lazy<()> = Lazy::new(|| {
    // setup logging for spawn app
    // trace debug info warn error
    let default_trace_name: String = "spawn_app".into();
    let default_level: String = "info".into();

    if env::var("RUST_LOG").is_ok() {
        //get subscriber for tracing
        let subscriber =
            telemetry::get_subscriber(default_trace_name, default_level, std::io::stdout);
        telemetry::init_subscriber(subscriber);
    } else {
        let subscriber =
            telemetry::get_subscriber(default_trace_name, default_level, std::io::sink);
        telemetry::init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    //configure database from config file
    let mut configuration =
        configuration::get_configuration().expect("Couldn't get configuration file");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&configuration.database).await;

    //let OS choose port for app and bind listener to it
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("unable to provide port to TCP listener");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let server = startup::run(listener, db_pool.clone()).expect("failed to bind address");
    let _ = tokio::spawn(server);

    // return app address to caller
    TestApp { address, db_pool }
}

async fn configure_database(config_db: &DataBaseSettings) -> PgPool {
    // Connect to docker container and create sqlx database
    let mut connection = PgConnection::connect_with(&config_db.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config_db.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Connect to docker container and migrate database
    let connection_pool = PgPool::connect_with(config_db.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let app_address = test_app.address;

    let client = Client::new();
    let response = client
        .get(&format!("{app_address}/health_check"))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await;

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = reqwest::Client::new()
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let client = Client::new();
    let test_app = spawn_app().await;
    let test_cases = [
        ("name=le%20guin", "missing email"),
        ("email=le%20guin%40gmail.com", "missing name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute post request with");

        assert_eq!(
            400,
            response.status().as_u16(),
            "app DID NOT fail with 400 when payload was {}",
            error_msg
        );
    }
}
