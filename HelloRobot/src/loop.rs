fn main() {
    println!("Loop example");

    let mut x = 0;

    loop {

        println!("x == {}", x);

        x += 1;

        if x ==10 {
            break;
        }
    }

    println!("x == {}", x);
}