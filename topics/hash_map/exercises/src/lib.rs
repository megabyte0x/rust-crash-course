use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut task: HashMap<String, u32> = HashMap::new();
    task.insert(address, amount);

    task
}
