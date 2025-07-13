use std::time::Duration;
use tokio::time::sleep;
use tokio::{join, select};

async fn get_eth_price_from_exchange_1() -> u32 {
    sleep(Duration::from_millis(1000)).await;
    1000
}

async fn get_eth_price_from_exchange_2() -> u32 {
    sleep(Duration::from_millis(1000)).await;
    1010
}

async fn get_btc_price_from_exchange_1() -> u32 {
    sleep(Duration::from_millis(500)).await;
    20000
}

#[tokio::main]
async fn main() {
    // Exercise 1
    let eth_price = get_eth_price_from_exchange_1().await;
    let btc_price = get_btc_price_from_exchange_1().await;

    println!("join: ETH price: {}", eth_price);
    println!("join: BTC price: {}", btc_price);

    // Exercise 2
    let eth_price = 0;
    println!("select: ETH price: {}", eth_price);
}
