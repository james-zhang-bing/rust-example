fn main() {
    grid(0.0003, 0.0002, 20, 6.89, 28920.0, 1000.0)
}


fn grid(
    upper: f64,
    lower: f64,
    grid_nums: usize,
    a_usdt_price: f64,
    b_usdt_price: f64,
    input_usdt: f64,
) {
    let current_price = a_usdt_price / b_usdt_price;
    if current_price>upper || current_price<lower{
        panic!("{}", format!("current_price>upper || current_price<lower current_price was {}",current_price));
    }
    let per_grid_height = (upper - lower) / grid_nums as f64;
    let mut on_price = vec![];
    let mut under_price = vec![];
    for v in 0..grid_nums + 1 {
        let p = lower + per_grid_height * v as f64;
        if p < current_price {
            under_price.push(p);
        } else if p <= upper {
            on_price.push(p)
        }
    }
    println!("on {:?} under:{:?}", on_price, under_price);
    println!("A/B");

    let num_of_order_sell_a = on_price.len();

    let p={
        let mut t=0.0;
        under_price.iter().for_each(|x|t=x+t);
        t
    };
    let a_per_order = {
        let divs=(num_of_order_sell_a as f64*a_usdt_price)+(p*b_usdt_price);
        input_usdt/divs
    };

    println!("trad a per order:{}",a_per_order);
    let need_a=num_of_order_sell_a as f64 *a_per_order;
    println!("need a coin:{}", need_a);
    println!("need usdt to buy a coin:{}",need_a*a_usdt_price);
    let need_b=a_per_order*p;
    println!("need b coin:{}", need_b);
    println!("need usdt to buy b coin:{}",need_b*b_usdt_price);
    println!("all need usdt :{}",need_a*a_usdt_price+need_b*b_usdt_price);

    
}

#[test]
fn test_grid() {
    grid(11.0, 1.0, 5, 8.0, 2.0, 1000.0);
}
