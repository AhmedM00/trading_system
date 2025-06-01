use std::{collections::HashMap, sync::{Arc, RwLock}};

use axum::{
    routing::{get, post}, Router
};
use state::AppState;

mod handlers;
mod state;
mod models;


#[tokio::main]
async fn main() {
    let app_state = AppState {
        trading_data: Arc::new(RwLock::new(HashMap::new())),
    };
    
    let app = Router::new()
        .route("/add_batch", post(handlers::add_batch_handler))
        .route("/stats", get(handlers::get_stats_handler))
        .with_state(app_state) ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}