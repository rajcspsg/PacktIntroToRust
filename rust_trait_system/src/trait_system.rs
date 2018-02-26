trait HasArea {
     fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64
}

impl HasArea for Square {
    fn area(&self) -> f64 {
            self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("area of the shape is {}",shape.area())
}


fn main() {
    let circle = Circle{x: 0.0f64, y:0.0f64, r:7.0f64};
    print_area(circle);
    let sqauare = Square{x:0.0f64, y:0.0f64, side: 5.0f64};
    print_area(sqauare);
}