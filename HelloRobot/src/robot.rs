struct Robot {

}

impl Robot {
    fn greet(&self) {
        println!("Hello developer!!!");
    }
    fn greet2() {
        println!("greet2 Hello developer!!!");
    }   
}

fn greet2() {
    println!("greet2 Hello developer!!!");
}

fn main() {
    println!("Helllo Robot!!!");

    let mut bot = Robot {};
    bot.greet();
    Robot::greet2();
    greet2();
}