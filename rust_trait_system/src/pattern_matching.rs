fn main() {
    let x = 5;

    let number = match x {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("other than 1 2 3"),
    };
    
    //println!("{}" , number);
}