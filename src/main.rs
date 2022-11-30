#[allow(dead_code)]
fn main() {
    let mut v = Vec::<String>::new();
    v.iter_mut().filter(|&ref a| a.is_empty());
    for &ref x in [&1] {}
    for &mut ref mut x in [&mut 1] {}
    for (i, &ref x) in [1].iter().enumerate() {}
}
