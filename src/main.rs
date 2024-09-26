use reqwest;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.bybit.com/v5/market/tickers?category=spot";
    let response = reqwest::get(url).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;

    let mut markets: Vec<String> = Vec::new();

    if let Some(list) = json["result"]["list"].as_array() {
        for item in list {
            if let Some(symbol) = item["symbol"].as_str() {
                if symbol.ends_with("BTC") && symbol != "BTCUSDT" {
                    markets.push(format!("BYBIT:{}", symbol));
                }
            }
        }
    }

    markets.sort_by(|a, b| {
        let a_numeric = a.chars().take_while(|c| c.is_numeric()).collect::<String>();
        let b_numeric = b.chars().take_while(|c| c.is_numeric()).collect::<String>();

        if !a_numeric.is_empty() && !b_numeric.is_empty() {
            b_numeric
                .parse::<i32>()
                .unwrap()
                .cmp(&a_numeric.parse::<i32>().unwrap())
        } else if !a_numeric.is_empty() {
            Ordering::Less
        } else if !b_numeric.is_empty() {
            Ordering::Greater
        } else {
            a.cmp(b)
        }
    });

    let mut file = File::create("bybit_btc_markets.txt")?;
    for market in markets {
        writeln!(file, "{}", market)?;
    }

    println!("Veriler başarıyla 'bybit_btc_markets.txt' dosyasına yazıldı.");
    Ok(())
}
