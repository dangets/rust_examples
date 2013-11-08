use std::f64;

struct Point {
    x: f64,
    y: f64
}

//enum Shape {
//    Circle { center: Point, radius: f64 },
//    Rectangle { top_left: Point, bottom_point: Point }
//}
//
//fn area(sh: Shape) -> f64 {
//    match sh {
//        Cicle { radius: radius, _ } => radius * radius,
//        Rectangle { top_left: top_left, bottom_right: bottom_right } => {
//            (bottom_right.x - top_left.x) * (top_left.y - bottom_right.y)
//        }
//    }
//}

enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point)
}

fn area(sh: Shape) -> f64 {
    match sh {
        Cicle(_, radius) => radius * radius,
        Rectangle(Point { x1, y1 }, Point { x2, y2 }) => (x2 - x1) * (y2 - y1)
    }
}


fn main() {
    println!("circle area: {}", area(
            Circle{Point{0.0, 0.0}, 10.0}
            ));
}
