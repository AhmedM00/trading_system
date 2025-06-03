use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use std::sync::{Arc, RwLock};
use tower::ServiceExt; 
use crate::{handlers::{add_batch_handler, get_stats_handler}, models::TradingData}; // Import handlers
use crate::state::AppState; 

/// create the application state
fn create_app_state() -> AppState {
    AppState {
        trading_data: Arc::new(RwLock::new(std::collections::HashMap::new())),
    }
}

/// create a router with handlers
fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/add_batch", axum::routing::post(add_batch_handler))
        .route("/stats", axum::routing::get(get_stats_handler))
        .with_state(app_state)
}

use axum::{response::Response};
use http_body_util::BodyExt; 
use bytes::Bytes;

async fn extract_body_bytes(response: Response<Body>) -> Bytes {
    let collected = response.into_body().collect().await.unwrap();
    collected.to_bytes()
}

#[tokio::test]
async fn test_add_batch_handler() {
    let app_state = create_app_state();
    let app = create_router(app_state.clone());

    let payload = r#"{"symbol": "AAPL", "values": [2.4, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0]}"#;

    let request = Request::builder()
        .uri("/add_batch")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(payload))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    
    // Assert response status
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_stats_handler() {

    let mut trading_data = TradingData::new();
    trading_data.add_batch(&[1.1, 2.7, 3.2, 4.9, 5.2, 11.7, 1.8, 8.4, 9.2, 12.4, 1.1]); // Add test data

    let app_state = AppState {
        trading_data: Arc::new(RwLock::new({
            let mut map = std::collections::HashMap::new();
            map.insert("AAPL".to_string(), trading_data);
            map
        })),
    };
    let app = create_router(app_state.clone());

    let request = Request::builder()
        .uri("/stats?symbol=AAPL&k=1")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    // Assert response status
    assert_eq!(response.status(), StatusCode::OK);

    // Parse the response body
    let body = extract_body_bytes(response).await;
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Assert stats values
    assert_eq!(response_json["min"], 1.1); 
    assert_eq!(response_json["max"], 12.4);
    assert_eq!(response_json["last"], 1.1);
    assert_eq!(response_json["avg"], 6.0600000000000005);
    assert_eq!(response_json["var"], 15.164399999999992);
}
