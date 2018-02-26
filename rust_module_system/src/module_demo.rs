mod robot {
    pub fn say_hello() {
        println!("Saying hello!!!");
    }

    pub fn say_hi() {
        println!("Saying hi!!!");
    }
}

fn main() {
     robot::say_hi();
    use robot::say_hello;
    say_hello();
   
}