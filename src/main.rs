pub enum A {
    B(i32),
    C(i32, i32, i32),
}

pub mod foo {
    pub fn bar() {}
}

fn main() {
    match A::B(1) {
        A::B(_) => println!("Matched"),
        _ => {
            println!("else block");
            return;
        }
    }
    // lint here
    match A::C(1, 2, 3) {
        A::C(_, _, _) => println!("Matched"),
        _ => {
            println!("else block");
            return;
        }
    }
}
