#[allow(dead_code)]
use std::sync::atomic::AtomicUsize;

fn main() {
    while 10 > 20 {
        const ATOM: AtomicUsize = AtomicUsize::new(0);
        println!("{:?}", ATOM);
    }
}
