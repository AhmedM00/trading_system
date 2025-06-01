use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};

use serde_json::{json};
use crate::{state::AppState};
use crate::models::{BatchRequest, StatsRequest, TradingData};

pub async fn add_batch_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<BatchRequest>
) -> impl IntoResponse{
    let mut state = app_state.trading_data.write().unwrap();
    let trading_data =state.entry(payload.symbol).or_insert(TradingData::new());
    
    trading_data.add_batch(&payload.values);

    let response = json!({"message": "batch is added successfully"});
    (StatusCode::OK, Json(response))
}


pub async fn get_stats_handler(
    State(app_state): State<AppState>,
    Query(payload): Query<StatsRequest>,
) -> impl IntoResponse{
    let state = app_state.trading_data.read().unwrap();
    let trading_data= state.get(&payload.symbol);

    match trading_data{
        Some(data) => {
            if let Some(stats_result) = data.get_stats(payload.k){
                (StatusCode::OK, Json(json!(stats_result)))
            }else{
                let error_response = json!({
                    "error": format!("data points to be analyzed is less than: {}", 10_i32.pow(payload.k as u32)),
                });
                (StatusCode::NOT_FOUND, Json(error_response))
            }
           
        },
        None => {
            let error_response = json!({
                "error": format!("no data found for symbol: '{}'", payload.symbol),
            });
            (StatusCode::NOT_FOUND, Json(error_response))
        }
    }
}