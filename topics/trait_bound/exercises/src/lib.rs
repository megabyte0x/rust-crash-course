use std::cmp::PartialOrd;

pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

pub fn zip<A: Copy, B: Copy>(a: Vec<A>, b: Vec<B>) -> Vec<(A, B)> {
    let mut v = vec![];
    let len = min(a.len(), b.len());

    for i in 0..len {
        v.push((a[i], b[i]));
    }

    v
}
