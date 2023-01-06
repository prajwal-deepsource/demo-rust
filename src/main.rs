struct A {
    a: i32,
    b: &'static str,
}

// This is an impl item for the A type
impl A {
    // This is a scammy default
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
