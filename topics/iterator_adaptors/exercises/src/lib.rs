use std::collections::HashMap;

pub fn filter_non_zero(v: Vec<u32>) -> Vec<u32> {
    let v: Vec<u32> = v.into_iter().filter(|x| *x > 0).collect();
    v
}

pub fn to_string(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

pub fn to_hash_map(v: Vec<(String, u32)>) -> HashMap<String, u32> {
    v.into_iter().collect()
}
