#[forbid(dead_code)]
use foo::bar;

fn main() {
    print!("Hello there\n");
    'x' as u8;
    ["A", "B"].iter().map(|x| x.clone());
    "Hello".splitn(2, 'l');
    let a: Rc<String> = Rc::new("hello".to_string());
    let b: Rc<&str> = Rc::new("hello");
    t.rewind();
    t.rewind();
}
