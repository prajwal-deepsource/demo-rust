use std::{
    io::{Seek, Write},
    mem,
};

enum SwarnimLint {
    A(i32),
    B(usize),
    C([usize; 1000]),
}

fn may_panic(v: Vec<i32>) -> Vec<i32> {
    v
}

#[allow(deprecated, invalid_value)]
fn myfunc(v: &mut Vec<i32>) {
    let taken_v = unsafe { mem::replace(v, mem::uninitialized()) };
    let new_v = may_panic(taken_v); // undefined behavior on panic
    mem::forget(mem::replace(v, new_v));
}

fn foo() -> std::io::Result<()> {
    let mut f = std::fs::File::create("foo.txt")?;
    f.write_all(b"Hello")?;
    eprintln!("Written {} bytes", f.seek(std::io::SeekFrom::Current(0))?);
    Ok(())
}

enum Foo {
    Bar,
}

fn baz() {
    // Foo::Bar is an enum variant
    let _ = Foo::Bar as usize;
}

fn bar() {}

fn bar_cast() {
    let fn_ptr: fn() = bar;
    if (fn_ptr as *const ()).is_null() {}
}

fn main() {
    let f = std::fs::File::create("foo.txt").unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_readonly(false);

    let x: Box<String> = Box::new(Default::default());
    let y = Box::new(i32::default());

    let bar = Some(3);
    if bar == None {
        // ...
    }
    if bar != None {
        // ...
    }
    let mut text = String::from("foo");
    let replaced = std::mem::replace(&mut text, String::default());

    let mut an_option = Some(0);
    let replaced = mem::replace(&mut an_option, None);
}
