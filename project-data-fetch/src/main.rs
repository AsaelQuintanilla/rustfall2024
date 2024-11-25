// use serde::Deserialize;
// use std::fs::OpenOptions;
// use std::io::Write;
// use std::thread;
// use std::time::Duration;
// use ureq;
// use serde_json;

// // Structs for Bitcoin and Ethereum
// #[derive(Debug)]
// struct Bitcoin {
//     api_address: String,
//     file_name: String,
// }

// #[derive(Debug)]
// struct Ethereum {
//     api_address: String,
//     file_name: String,
// }

// // Trait to Standardize Pricing Fetch and Save Mechanisms
// pub trait Pricing {
//     fn fetch_price(&self) -> Option<f32>;
//     fn save_to_file(&self, price: f32);
// }

// // Deserialize structs for parsing API responses
// #[derive(Debug, Deserialize)]
// struct PriceResponse {
//     bitcoin: Option<AssetPrice>,
//     ethereum: Option<AssetPrice>,
// }

// #[derive(Debug, Deserialize)]
// struct AssetPrice {
//     usd: f32,
// }

// // Implement Pricing trait for Bitcoin
// impl Pricing for Bitcoin {
//     fn fetch_price(&self) -> Option<f32> {
//         let response = ureq::get(&self.api_address).call();
//         if response.is_ok() {
//             let response = response.unwrap(); // Safe since we checked `is_ok`
//             if let Ok(parsed) = serde_json::from_reader::<_, PriceResponse>(response.into_reader()) {
//                 return Some(parsed.bitcoin.unwrap().usd as f32);
//             }
//         }
//         None
//     }

//     fn save_to_file(&self, price: f32) {
//         let data = format!(
//             "{{\"price\": {:.2}, \"timestamp\": \"{}\"}}\n",
//             price,
//             chrono::Utc::now()
//         );
//         let mut file = OpenOptions::new()
//             .create(true)
//             .append(true)
//             .open(&self.file_name)
//             .expect("Unable to open file");
//         file.write_all(data.as_bytes())
//             .expect("Unable to write to file");
//     }
// }

// impl Pricing for Ethereum {
//     fn fetch_price(&self) -> Option<f32> {
//         let response = ureq::get(&self.api_address).call();
//         if response.is_ok() {
//             let response = response.unwrap(); // Safe since we checked `is_ok`
//             if let Ok(parsed) = serde_json::from_reader::<_, PriceResponse>(response.into_reader()) {
//                 return parsed.ethereum.map(|asset| asset.usd);
//             }
//         }
//         None
//     }

//     fn save_to_file(&self, price: f32) {
//         let data = format!(
//             "{{\"price\": {:.2}, \"timestamp\": \"{}\"}}\n",
//             price,
//             chrono::Utc::now()
//         );
//         let mut file = OpenOptions::new()
//             .create(true)
//             .append(true)
//             .open(&self.file_name)
//             .expect("Unable to open file");
//         file.write_all(data.as_bytes())
//             .expect("Unable to write to file");
//     }
// }

// // Main Function with Periodic Execution
// fn main() {
//     // Create instances of Bitcoin and Ethereum
//     let bitcoin = Bitcoin {
//         api_address: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd"
//             .to_string(),
//         file_name: "btc_prices.json".to_string(),
//     };

//     let ethereum = Ethereum {
//         api_address: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd"
//             .to_string(),
//         file_name: "eth_prices.json".to_string(),
//     };

//     // Periodic fetch and save loop
//     loop {
//         // Fetch and save Bitcoin price
//         if let Some(price) = bitcoin.fetch_price() {
//             bitcoin.save_to_file(price);
//             println!("Bitcoin price saved: {:.2}", price);
//         } else {
//             println!("Failed to fetch Bitcoin price.");
//         }

//         // Fetch and save Ethereum price
//         if let Some(price) = ethereum.fetch_price() {
//             ethereum.save_to_file(price);
//             println!("Ethereum price saved: {:.2}", price);
//         } else {
//             println!("Failed to fetch Ethereum price.");
//         }

//         // Wait for 10 seconds before fetching again
//         thread::sleep(Duration::from_secs(10));
//     }
// }
// ^
// |
// |
// works
/////////////////////////////////////////////////////////////////////////

use chrono::Utc;
use serde::{Deserialize};
use std::{fs::OpenOptions, io::Write};
use ureq;

#[derive(Debug, Deserialize)]
struct Cost {
    usd: f32,
}

#[derive(Debug, Deserialize)]
struct PriceResponse {
    bitcoin: Option<Cost>,
    ethereum: Option<Cost>,
}

#[derive(Debug, Deserialize)]
struct SP500Response {
    chart: SP500Chart,
}

#[derive(Debug, Deserialize)]
struct SP500Chart {
    result: Vec<SP500Result>,
}

#[derive(Debug, Deserialize)]
struct SP500Result {
    meta: SP500Meta,
}

#[derive(Debug, Deserialize)]
struct SP500Meta {
    regular_market_price: f32,
}

#[derive(Debug)]
struct Bitcoin {
    api_address: String,
    file_name: String,
}

#[derive(Debug)]
struct Ethereum {
    api_address: String,
    file_name: String,
}

#[derive(Debug)]
struct SP500 {
    api_address: String,
    file_name: String,
}

pub trait Pricing {
    fn fetch_price(&self) -> Option<f32>;
    fn save_to_file(&self, price: f32);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Option<f32> {
        let response = ureq::get(&self.api_address).call();
        if response.is_ok() {
            let response = response.unwrap();
            if let Ok(parsed) = serde_json::from_reader(response.into_reader()) as Result<PriceResponse, _> {
                return parsed.bitcoin.map(|asset| asset.usd);
            }
        }
        None
    }

    fn save_to_file(&self, price: f32) {
        let data = format!(
            "{{\"price\": {:.2}, \"timestamp\": \"{}\"}}\n",
            price,
            Utc::now()
        );
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .expect("Unable to open file");
        file.write_all(data.as_bytes())
            .expect("Unable to write to file");
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Option<f32> {
        let response = ureq::get(&self.api_address).call();
        if response.is_ok() {
            let response = response.unwrap();
            if let Ok(parsed) = serde_json::from_reader(response.into_reader()) as Result<PriceResponse, _> {
                return parsed.ethereum.map(|asset| asset.usd);
            }
        }
        None
    }

    fn save_to_file(&self, price: f32) {
        let data = format!(
            "{{\"price\": {:.2}, \"timestamp\": \"{}\"}}\n",
            price,
            Utc::now()
        );
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .expect("Unable to open file");
        file.write_all(data.as_bytes())
            .expect("Unable to write to file");
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Option<f32> {
        let response = ureq::get(&self.api_address).call();
        if response.is_ok() {
            let response = response.unwrap();
            if let Ok(parsed) = serde_json::from_reader(response.into_reader()) as Result<SP500Response, _> {
                return Some(parsed.chart.result[0].meta.regular_market_price);
            }
        }
        None
    }

    fn save_to_file(&self, price: f32) {
        let data = format!(
            "{{\"price\": {:.2}, \"timestamp\": \"{}\"}}\n",
            price,
            Utc::now()
        );
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .expect("Unable to open file");
        file.write_all(data.as_bytes())
            .expect("Unable to write to file");
    }
}

fn main() {
    let btc = Bitcoin {
        api_address: "https://api.coindesk.com/v1/bpi/currentprice/BTC.json".to_string(),
        file_name: "btc_prices.json".to_string(),
    };

    let eth = Ethereum {
        api_address: "https://api.coindesk.com/v1/bpi/currentprice/ETH.json".to_string(),
        file_name: "eth_prices.json".to_string(),
    };

    let sp500 = SP500 {
        api_address: "https://query1.finance.yahoo.com/v8/finance/chart/^GSPC?interval=1m".to_string(),
        file_name: "sp500_prices.json".to_string(),
    };

    loop {
        if let Some(price) = btc.fetch_price() {
            btc.save_to_file(price);
        }

        if let Some(price) = eth.fetch_price() {
            eth.save_to_file(price);
        }

        if let Some(price) = sp500.fetch_price() {
            sp500.save_to_file(price);
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
