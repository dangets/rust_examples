use std::f64;
use std::num::atan;

fn angle(vector: (f64, f64)) -> f64 {
    let pi = f64::consts::PI;
    match vector {
        (0.0, y) if y < 0.0 => 1.5 * pi,
        (0.0, y) => 0.5 * pi,
        (x, y) => atan(y / x)
    }
}


fn main() {
    println!("{} -> {}", (1.0, 1.0), angle((1.0, 1.0)));
}
