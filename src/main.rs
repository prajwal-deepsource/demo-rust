use std::ptr;

fn main() {
    let a: *const i32 = ptr::null();
    if a == ptr::null() {
        println!("Breh");
    }
    if a != ptr::null() {
        println!("Bruh");
    }
}
