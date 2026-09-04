//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.

// You can also inspect what code gets generated using `cargo expand --test health_check` (<- name of the test file).
use std::net::TcpListener;

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind address.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener)
        .expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{port}")
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `request`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(format!("http://{address}/health_check"))
        .send()
        .await
        .expect("Falied to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_success() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
 
    // Act
    let body = "name=Niloy%20Hurayra&email=niloyhurayra%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_failed() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        // Act 
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert 
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customized error message on test failure
            "The api did not fail with 400 bad request when the payload was {}.",
            error_message
        );
    }
}


