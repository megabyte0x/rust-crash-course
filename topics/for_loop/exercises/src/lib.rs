pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in nums.iter() {
        sum += i;
    }
    return sum;
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut v: Vec<u32> = vec![i; n];

    return v;
}
