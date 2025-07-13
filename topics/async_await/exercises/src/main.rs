use std::time::Duration;
use tokio::time::sleep;

fn hello() {
    println!("Hello async Rust!");
}

fn add(x: u32, y: u32) -> u32 {
    sleep(Duration::from_millis(1000));
    x + y
}

fn main() {
    hello();

    let sum = add(1, 2);
    println!("sum: {sum}");
}
