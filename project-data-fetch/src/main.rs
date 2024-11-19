#[derive(Debug)]
struct Bitcoin{
    api_address:String,
    file_name:String,
}

struct Etherium{
    api_address:String,
    file_name:String,
}

struct SP500{
    api_address:String,
    file_name:String,
}

pub trait pricing{
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self);
}

impl Pricing for Bitcoin{
    fn fetch_price(&self) -> f32 {
        return 32.00;
    }
    fn save_to_file(&self) {
        println!("saved to {}", self.file_name);
    }
}

#[derive(Debug,Deserialize)]
struct Cost {
    usd: i32;
}

#[derive(Debug,Deserialize)]
struct BTCPriceAPI {
    bitcoin: Cost,
}

use serde::Deserialize;

fn main() {
    let btc_api: String = 
    let btc_txt: String = "btc_prices.json".to_string();
    let b = Bitcoin = Bitcoin(api_address:btc_api,file_name:btc_txt);

    let btc_price: Response = ureq::get(path: &b.api_address).call().umwrap();

    let b: BTCPriceAPI = btc_price.info_json::<BTCPriceAPI>().unwrap();

    println!("{:?}",b);
}