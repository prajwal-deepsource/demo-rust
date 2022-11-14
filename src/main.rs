#[allow(dead_code)]
use std::sync::atomic::AtomicUsize;

enum EnumABC {
    A(i32),
    B(u32),
    C(&'static str),
}

fn bar() -> EnumABC {
    EnumABC::B(10u32)
}

fn foo() {
    let EnumABC::A(_) = bar() else { return };
}

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
