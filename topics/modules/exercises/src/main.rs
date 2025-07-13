mod math {
    pub fn min(x: u32, y: u32) -> u32 {
        if x <= y { x } else { y }
    }

    pub fn max(x: u32, y: u32) -> u32 {
        if x >= y { x } else { y }
    }
}

mod util {
    pub mod log {
        pub fn debug(s: &str) {
            println!("DEBUG: {s}");
        }
    }

    pub mod vec {
        pub fn first(v: &[u32]) -> Option<u32> {
            let n = v.len();
            if n > 0 { Some(v[0]) } else { None }
        }

        pub fn last(v: &[u32]) -> Option<u32> {
            let n = v.len();
            if n > 0 { Some(v[n - 1]) } else { None }
        }
    }
}

fn main() {
    util::log::debug(&format!("min: {}", math::min(1, 2)));
    util::log::debug(&format!("max: {}", math::max(1, 2)));
}
