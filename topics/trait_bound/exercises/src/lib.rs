use std::cmp::PartialOrd;

pub fn min(x: u32, y: u32) -> u32 {
    if x <= y { x } else { y }
}

pub fn zip(a: Vec<u32>, b: Vec<i32>) -> Vec<(u32, i32)> {
    let mut v = vec![];
    let len = min(a.len(), b.len());

    for i in 0..len {
        v.push((a[i], b[i]));
    }

    v
}
