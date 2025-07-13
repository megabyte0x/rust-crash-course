#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    todo!();
}

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    todo!();
}
