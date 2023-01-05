struct A {
    a: i32,
    b: &'static str,
}

impl A {
    fn default() -> A {
        A { a: 0, b: "Hola" }
    }
    fn print_a(&self) {
        println!("{}", self.a);
    }
    fn print_b(&self) {
        println!("{}", self.b);
    }
}

fn main() {}
