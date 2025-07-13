#[derive(Debug)]
struct Config<'a> {
    url: &'a str,
    port: u32,
    db_url: &'a str,
}

// Lifetime 'a can be omitted. Rust can automatically figure out the lifetime
fn log(config: &Config) {
    println!("Connecting to database at {}...", config.db_url);
    println!("server listening at {}:{}...", config.url, config.port);
}

fn main() {
    let config = Config {
        url: "localhost",
        port: 3000,
        db_url: "db://localhost",
    };

    log(&config);
}
