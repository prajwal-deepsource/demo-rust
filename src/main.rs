use std::ptr;

#[allow(unused)]
fn main() {
    let a: *const i32 = ptr::null();
    if a == ptr::null() {
        println!("Breh");
    }
    if a != ptr::null() {
        println!("Bruh");
    }

    let b = String::new();
    let c: Vec<i32> = Vec::new();
}
