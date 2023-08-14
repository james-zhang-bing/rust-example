use std::{vec, ops::Range};

use rand::Rng;
fn main() {
    let s=Simulator::new();
    s.run(10000);
}

struct Simulator {
    mutation_rate:Range<f64>,
    final_percent:f64,
    coin_price:Vec<f64>,
    asset_ratio:Vec<f64>,
    init_asset:f64,
}


impl Simulator{
    fn run(&self,times:u64){
        let mut rng=rand::thread_rng();
        let mut current_price=self.coin_price.clone();
        let ratio_sum:f64=self.asset_ratio.iter().sum();
        let mut asset=vec![];
        for (i,ratio) in self.asset_ratio.iter().enumerate(){
            asset.push(self.init_asset*(ratio/ratio_sum)/current_price[i]);
        }

        for i in 0..times {

            //refresh new price
            for price_num in 0..current_price.len(){
                let direct=rng.gen_range(self.mutation_rate.clone());
                current_price[price_num]=direct*current_price[price_num];
            }

            let total_value={
                let mut sum=0.0;
                for (n,v) in asset.iter().enumerate(){
                    sum+=v*current_price[n];
                }
                sum
            };

            //new asset
            asset=vec![];
            for (i,ratio) in self.asset_ratio.iter().enumerate(){
                asset.push(total_value*(ratio/ratio_sum)/current_price[i]);
            }
            
            
        }
        let mut final_price=vec![];
        for v in self.coin_price.iter(){
            final_price.push(v*self.final_percent);
        }

        let total_value={
            let mut sum=0.0;
            for (n,v) in asset.iter().enumerate(){
                sum+=v*final_price[n];
            }
            sum
        };
        let mut asset_value=vec![];
        for (n,v) in asset.iter().enumerate(){
            asset_value.push(v*final_price[n]);
        }
        println!("total value was {}",total_value);
        println!("asset {:?}",asset);
        println!("asset value {:?}",asset_value);

    }

    fn new()->Self{
        Self{
            asset_ratio:vec![10.0,8.0,5.0,3.0],
            coin_price:vec![30000.0,2000.0,20.0,10.0],
            final_percent:0.6,
            init_asset:10000.0,
            mutation_rate:0.98..1.02,
        }
    }
}