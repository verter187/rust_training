fn main() {
    let mut a = 0;
    for _ in 0..1000000000 {
        a += 1;
    }
    println!("num1={:?}", a);
}
