pub fn first<A, B>(t: (A, B)) -> A {
    t.0
}

pub fn last<A, B>(t: (A, B)) -> B {
    t.1
}

#[derive(Debug)]
pub struct Rectangle<A> {
    pub top: A,
    pub left: A,
    pub width: A,
    pub height: A,
}
