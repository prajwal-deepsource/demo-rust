#[allow(dead_code)]
use std::sync::atomic::AtomicUsize;

#[repr(usize)]
enum Test {
    X = 0x1_000_000_000,
    Y,
}

fn main() {
    while 100 > 20 {
        const ATOM: AtomicUsize = AtomicUsize::new(0);
        println!("{:?}", ATOM);
    }
}
