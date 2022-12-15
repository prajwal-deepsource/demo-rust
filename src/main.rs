use std::ptr;

#[allow(dead_code)]
struct A {}

impl A {
    fn default() -> Self {
        Self {}
    }
}

fn main() {
    let a = i32::default();
    let b = a.clone();
    let c = 0 as *const i32;
    if c != ptr::null() {
        println!("Not null");
    }
    if c == ptr::null() {
        println!("Null");
    }
}
