#[allow(dead_code)]
use std::rc::Rc;

fn main() {
    let mut v = Vec::new();
    let r = Rc::new(10);
    v.push(r.clone());
}
