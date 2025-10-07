mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_home_endpoint() {
    let app = common::setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    assert!(body_str.contains("Welcome to auth_session API"));
}

#[tokio::test]
async fn test_register_success() {
    let app = common::setup_test_app().await;
    
    let email = common::get_test_email();
    let payload = json!({
        "email": email,
        "password": "Test123!@#",
        "name": "Test User"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(body_json["token"].is_string());
    assert_eq!(body_json["user"]["email"], email);
    assert_eq!(body_json["user"]["name"], "Test User");
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let app = common::setup_test_app().await;
    
    let email = common::get_test_email();
    let payload = json!({
        "email": email,
        "password": "Test123!@#",
        "name": "Test User"
    });

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_client_error() || response.status().is_server_error());
}

#[tokio::test]
async fn test_login_success() {
    let app = common::setup_test_app().await;
    
    let email = common::get_test_email();
    let password = "Test123!@#";
    let register_payload = json!({
        "email": email,
        "password": password,
        "name": "Test User"
    });

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let login_payload = json!({
        "email": email,
        "password": password
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(body_json["token"].is_string());
    assert_eq!(body_json["user"]["email"], email);
}

#[tokio::test]
async fn test_login_wrong_password() {
    let app = common::setup_test_app().await;
    
    let email = common::get_test_email();
    let register_payload = json!({
        "email": email,
        "password": "CorrectPassword123",
        "name": "Test User"
    });

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let login_payload = json!({
        "email": email,
        "password": "WrongPassword123"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    let app = common::setup_test_app().await;
    
    let login_payload = json!({
        "email": "nonexistent@example.com",
        "password": "SomePassword123"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_logout() {
    let app = common::setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/logout")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(body_json["message"].is_string());
}
