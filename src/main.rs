pub struct A {
    B {
        a: i32,
        b: u32,
    },
    pub C: u32,
}

pub enum D {
    UnitE,
    E(i32),
    F(u32),
}

pub union G {
    H: i32,
    pub I: u32,
}

pub trait TestDocs {
    const TEST: usize;
    type Test;
    fn test();
}

impl A {
    pub const TEST: usize = 42;
    pub fn test() {}
}

pub static s: &str = "hello world";

pub const ANSWER: usize = 42;

pub type Test = (usize, usize);

pub mod foo;

pub fn test() {}

/// This is a doc comment
pub struct A {
    B: i32,
    /** This is also a doc comment */
    pub C: u32,
}

struct privateA {}

#[doc = "This is a doc comment replacement"]
pub enum D {
#[doc(hidden)]
    E(i32),
    #[doc = "This is a doc comment replacement"]
    F(u32),
}

enum privateD {
    privateE,
    privateF(u32),
}

#[doc(hidden)]
pub union G {
    H: i32,
    #[doc(hidden)]
    pub I: u32,
}

union privateE {}

#[doc(hidden)]
pub trait TestDocs {
    const TEST: usize;
    type Test;
    fn test();
}

impl A {
    #[doc(hidden)]
    pub const TEST: usize = 42;
    #[doc(hidden)]
    pub fn test() {}
}

#[doc(hidden)]
pub static s: &str = "hello world";

#[doc(hidden)]
pub const ANSWER: usize = 42;

#[doc(hidden)]
pub type Test = (usize, usize);

#[doc(hidden)]
pub mod foo;

#[doc(hidden)]
pub fn test() {}
