pub fn hello() -> String {
    let s: String = "Hello Rust".to_string();
    s
}

pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn append(mut s: String) -> String {
    s += "!";
    s
}
