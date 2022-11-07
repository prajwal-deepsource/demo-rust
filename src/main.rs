#[allow(dead_code)]
use std::sync::atomic::AtomicUsize;

fn main() {
    while 100 > 20 {
        const ATOM: AtomicUsize = AtomicUsize::new(0);
        println!("{:?}", ATOM);
    }

    while true {
        println!("potato");
    }

    while 20 < 5 {
        println!("mega potato");
    }
}
