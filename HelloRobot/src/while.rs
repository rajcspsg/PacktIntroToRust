fn main() {
    let mut a = 5;
    let mut b = 1;

    while a == 5 {
        b += 1;
        if b == 5 {
            a += 1;
        }
    }

    println!("a == {}", a);
    println!("b == {}", b);
}