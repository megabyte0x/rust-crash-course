#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
}
