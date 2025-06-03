use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

const MAX_POINTS_SIZE: i32 = 10_i32.pow(8);

fn round_to_4_decimals(value: f64) -> f64 {
    (value * 10000.0).round() / 10000.0
}

#[derive(Deserialize)]
pub struct BatchRequest{
    pub symbol: String,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub struct StatsRequest{
    pub symbol: String,
    pub k: i32,
}

#[derive(Debug, Serialize)]
pub struct StatsResult {
    pub min: f64,
    pub max: f64,
    pub last: f64,
    pub avg: f64,
    pub var: f64,
}


//TradingData is the struct that holds
//trading data points in "data_points" varient
//trading data stats in "data_stats" varient
#[derive(Default, Debug, Clone, Serialize)]
pub struct TradingData{
    pub data_points: VecDeque<f64>,
    data_stats: Vec<DataStats>,
}
#[derive(Debug, Clone, Serialize)]
struct DataStats{
    min: f64,
    max: f64,
    sum: f64,
    sum_of_squared: f64,
}

impl TradingData{

    pub fn new() -> Self{
        Self { 
            data_points: VecDeque::new(), 
            data_stats: vec![DataStats { min: f64::MAX, max: 0.0, sum: 0.0, sum_of_squared: 0.0 }; 8]
        }
    }

    pub fn add_batch(&mut self, batch: &[f64]){

        let points_len = self.data_points.len();
        
        //iterate over the batch to be added
        for (i, &value) in batch.iter().enumerate(){

            if (i + points_len + 1) as i32 > MAX_POINTS_SIZE{
                self.data_points.pop_front();
            }

            //push the batch data to data_points vector
            self.data_points.push_back(value);

        }

        //Initialize variables needed to calculate the stats
        let mut min = f64::MAX;
        let mut max = 0_f64;
        let mut sum = 0_f64;
        let mut sum_of_squared = 0_f64;

        for (i, &value) in self.data_points.iter().rev().enumerate() {
            let index = (i as f64).log10() as usize;
            let stats = &mut self.data_stats[index];
            
            //compare last initialized variables with the new batch data
            min = min.min(value);
            max = max.max(value);
            sum += value;
            sum_of_squared += value * value;

            //update the data_stats
            stats.min = min;
            stats.max = max;
            stats.sum = sum;
            stats.sum_of_squared = sum_of_squared;

        }
    }

    pub fn get_stats(&self, k: i32) -> Option<StatsResult>{
        
        //k_len is 10e{k}
        let k_len = (10_i32.pow(k as u32)) as f64;

        //index is k-1 because we start from 0 to index 7 and we could have k equals 1 to 8 
        let index = (k - 1) as usize;

        //return None if we don't have any data_points 
        //OR if 10e{k} = amount of data we need to analyze bigger than the amount of data we have in out data_points
        if self.data_points.back().is_none() || k_len as usize > self.data_points.len() {
            return None;
        }

        //calculate the wanted data (min, max, last, avg, var)
        let min = self.data_stats[index].min;
        let max = self.data_stats[index].max;
        let last = *self.data_points.back().unwrap();

        let sum = self.data_stats[index].sum;
        let avg = sum / k_len;

        let sum_of_squared = self.data_stats[index].sum_of_squared;
        let var = (sum_of_squared / k_len) - (avg * avg);

        Some(StatsResult {
            min,
            max,
            last,
            avg: round_to_4_decimals(avg),
            var: round_to_4_decimals(var),
        })
    }
}