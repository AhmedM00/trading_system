use std::{collections::HashMap, sync::Arc};
use std::sync::RwLock;

use crate::models::TradingData;

#[derive(Clone)]
pub struct AppState {
    pub trading_data: Arc<RwLock<HashMap<String, TradingData>>>,
}