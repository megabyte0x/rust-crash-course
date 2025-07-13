#![allow(unused)]

fn print_len(s: String) {
    println!("len: {}", s.len());
}

fn main() {
    // Exercise 1
    let s = String::from("Rust");
    let s1 = &mut s;
    let s2 = &mut s;

    println!("s1: {s1}");
    println!("s2: {s2}");

    // Exercise 2
    let mut s = String::from("Rust");
    let s1 = &mut s;
    let s2 = &mut s;

    s1.push_str("!!!");
    println!("s: {s}");

    // Exercise 3
    let s = String::from("Rust");
    print_len(s);
    println!("s: {s}");
}
