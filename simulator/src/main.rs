use rand::Rng;
use std::{ops::RangeInclusive, vec};
fn main() {
    let s = Simulator::new();
    let yields = s.run(1000);
    let sharpe_rate = sharpe_ratio(yields, 1.0);
    println!("sharpe ratio {}", sharpe_rate);
}

struct Simulator {
    mutation_rate: RangeInclusive<f64>,
    final_percent: f64,
    coin_price: Vec<f64>,
    asset_ratio: Vec<f64>,
    init_asset: f64,
}

impl Simulator {
    fn run(&self, times: u64) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let mut current_price = self.coin_price.clone();
        let ratio_sum: f64 = self.asset_ratio.iter().sum();
        let mut asset = vec![];
        for (i, ratio) in self.asset_ratio.iter().enumerate() {
            asset.push(self.init_asset * (ratio / ratio_sum) / current_price[i]);
        }
        let mut each_earning_yield = vec![];
        let mut latest_total_value = self.init_asset;
        for i in 0..times {
            //refresh new price
            for price_num in 0..current_price.len() {
                let direct = rng.gen_range(self.mutation_rate.clone());
                current_price[price_num] = direct * current_price[price_num];
            }

            let total_value = {
                let mut sum = 0.0;
                for (n, v) in asset.iter().enumerate() {
                    sum += v * current_price[n];
                }
                sum
            };
            each_earning_yield
                .push((total_value - latest_total_value) * 100.0 / latest_total_value);
            latest_total_value = total_value;
            //new asset
            asset = vec![];
            for (i, ratio) in self.asset_ratio.iter().enumerate() {
                asset.push(total_value * (ratio / ratio_sum) / current_price[i]);
            }
        }
        let total_value = {
            let mut sum = 0.0;
            for (n, v) in asset.iter().enumerate() {
                sum += v * current_price[n];
            }
            sum
        };
        println!("current price {:?}", current_price);
        println!("current value {}", total_value);
        let mut final_price = vec![];
        for v in self.coin_price.iter() {
            final_price.push(v * self.final_percent);
        }

        let total_value = {
            let mut sum = 0.0;
            for (n, v) in asset.iter().enumerate() {
                sum += v * final_price[n];
            }
            sum
        };
        let mut asset_value = vec![];
        for (n, v) in asset.iter().enumerate() {
            asset_value.push(v * final_price[n]);
        }
        println!("total value was {}", total_value);
        println!("asset {:?}", asset);
        println!("asset value {:?}", asset_value);
        each_earning_yield
    }

    fn new() -> Self {
        Self {
            asset_ratio: vec![10.0, 8.0, 5.0, 3.0],
            coin_price: vec![30000.0, 2000.0, 20.0, 10.0],
            final_percent: 1.5,
            init_asset: 10000.0,
            mutation_rate: 0.99..=1.03,
        }
    }
}

fn sharpe_ratio(each_earnings_yield: Vec<f64>, risk_free_interest_rate: f64) -> f64 {
    let rounds = each_earnings_yield.len();
    if rounds == 0 {
        return 0.0;
    }
    let mean = each_earnings_yield.iter().sum::<f64>() / each_earnings_yield.len() as f64;
    println!("mean:{}", mean);
    let variance = each_earnings_yield
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>()
        / (each_earnings_yield.len() - 1) as f64;
    let std_dev = variance.sqrt();
    println!("std_dev:{}", std_dev);
    (mean - risk_free_interest_rate) / std_dev
}
