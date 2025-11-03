//! tests/health_check.rs


use std::net::TcpListener;
use reqwest::Client;
use zero2prod::startup;

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("unable to provide port to TCP listener");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener)
        .expect("failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    // return app address to caller
    format!("http://127.0.0.1:{port}")
}


// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using 
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app();
    let client = Client::new();
    let response = client.get(&format!("{app_address}/health_check")).send().await.expect("failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let client = Client::new();
    let app_address = spawn_app();
    let test_cases = [
        ("name=le%20guin", "missing email"),
        ("email=le%20guin%40gmail.com", "missing name"),
        ("", "missing both")];
        //("name=le%20guin&email=ursula_le_guin%40gmail.com", "CORRECT NAME AND EMAIL")

    for (invalid_body, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
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

