use std::float::consts::pi;

struct Point {
    x: float,
    y: float,
}

struct Circle {
    origin: Point,
    radius: float,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Circle {
    fn new(radius: float) -> Circle {
        Circle {
            origin: Point { x: 0.0, y: 0.0 },
            radius: radius,
        }
    }

    fn area(&self) -> float {
        pi * self.radius * self.radius
    }
}


trait Areable {
    fn area(&self) -> float;
}

impl Areable for Circle {
    fn area(&self) -> float {
        self.area()
    }
}


fn print_area<T: Areable>(a: &T) {
    println!("{}", a.area());
}


fn main() {
    let c = Circle::new(42.5);
    println!("{}", c.area());

    print_area(&c);
}

//impl Areable for Circle {
//}
