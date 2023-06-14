fn main() {
    let mut v = vec![1, 2, 3, 4];
    v = v.into_iter().filter(|&e| e != 2).collect();
    let v2: Vec<i32> = v.into_iter().filter(|&e| e != 2).collect();
}
