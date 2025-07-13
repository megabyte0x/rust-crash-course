pub trait Tester {
    fn test(&self, file_path: &str) -> String;
}

pub struct Foundry {
    pub version: String,
}

pub struct Cargo {
    pub version: String,
}

impl Tester for Foundry {
    fn test(&self, file_path: &str) -> String {
        format!("forge test {}", file_path)
    }
}

impl Tester for Cargo {
    fn test(&self, file_path: &str) -> String {
        format!("cargo test {}", file_path)
    }
}

pub fn test(tester: &impl Tester, file_path: &str) -> String {
    tester.test(file_path)
}
