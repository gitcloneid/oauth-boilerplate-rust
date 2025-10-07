mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_profile_with_valid_token() {
    let app = common::setup_test_app().await;
    
    let email = common::get_test_email();
    let payload = json!({
        "email": email,
        "password": "Test123!@#",
        "name": "Test User"
    });

    let register_response = app
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

    let body = axum::body::to_bytes(register_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let token = body_json["token"].as_str().unwrap();

    let profile_response = app
        .oneshot(
            Request::builder()
                .uri("/api/profile")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(profile_response.status(), StatusCode::OK);
    
    let profile_body = axum::body::to_bytes(profile_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let profile_json: serde_json::Value = serde_json::from_slice(&profile_body).unwrap();
    
    assert_eq!(profile_json["email"], email);
    assert_eq!(profile_json["name"], "Test User");
    assert!(profile_json["id"].is_string());
}

#[tokio::test]
async fn test_get_profile_without_token() {
    let app = common::setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/profile")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_profile_with_invalid_token() {
    let app = common::setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/profile")
                .header("authorization", "Bearer invalid_token_here")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_profile_with_malformed_auth_header() {
    let app = common::setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/profile")
                .header("authorization", "InvalidFormat")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
