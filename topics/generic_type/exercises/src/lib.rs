pub fn first(t: (u32, i32)) -> u32 {
    t.0
}

pub fn last(t: (u32, i32)) -> i32 {
    t.1
}

#[derive(Debug)]
pub struct Rectangle {
    pub top: u32,
    pub left: u32,
    pub width: u32,
    pub height: u32,
}
