#[allow(dead_code)]
use std::path::Path;

fn main() {
    println!("{}", Path::new("/deepsource/test-rust").display());
}
