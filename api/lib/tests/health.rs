use actix_web::{http::StatusCode, App};
use api_lib::health::{service, API_VERSION};
use reqwest::Client;

/// Test for the health check endpoint from Rust Full Stack Workshop
#[actix_rt::test]
async fn health_check_works() {
    let app = App::new().configure(service);
    let mut app = actix_web::test::init_service(app).await;
    let req = actix_web::test::TestRequest::get()
    .uri("/health")
    .to_request();

    let resp = actix_web::test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.status().is_success(), true);
    let data = resp
        .headers()
        .get("version")
        .and_then(|v| v.to_str().ok());
    assert_eq!(data, Some(API_VERSION));
}

// Test for 404 invalid path, generated from GitHub Copilot
#[actix_rt::test]
async fn health_check_returns_404_for_invalid_path() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get(&format!("{}/invalid-path", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

// Test for idempotency of the health check endpoint, generated from GitHub Copilot
// to print out with println!() during testing it is necessary to start
// the test with `cargo test -- --nocapture`
#[actix_rt::test]
async fn health_check_is_idempotent() {
    // Arrange
    let app_address = spawn_app();
    println!("App address: {}", app_address);
    let client = Client::new();

    // Act
    let response1 = client
        .get(&format!("{}/health", app_address))
        .send()
        .await
        .expect("Failed to execute request.");
    println!("Response 1: {:?}", response1);
    let response2 = client
        .get(&format!("{}/health", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response1.status().is_success());

    assert!(response2.status().is_success());
}

// Helper function to spawn the application
fn spawn_app() -> String {
    // Assuming your application runs on localhost:8000
    // Replace with dynamic port binding if needed
    "http://127.0.0.1:8000".to_string()
}