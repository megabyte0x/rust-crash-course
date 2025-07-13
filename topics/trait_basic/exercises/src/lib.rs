pub trait Tester {
    fn test(&self, file_path: &str) -> String;
}

pub struct Foundry {
    pub version: String,
}

pub struct Cargo {
    pub version: String,
}

pub fn test(tester: ?, file_path: &str) -> String {
    todo!();
}
