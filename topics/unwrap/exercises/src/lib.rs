pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let newA: u32 = a.parse().expect("Failed to parse variable");
    let newB: u32 = b.parse().expect("Failed to parse variable");

    newA + newB
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    x.unwrap() + y.unwrap()
}
