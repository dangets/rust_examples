

fn line(a: int, b: int, x: int) -> int {
    a * x + b
}

fn oops(a: int, b: int, x: int) -> () {
    a * x + b;
}

fn main() {
    assert!(line(5, 3, 1) == 8);
    assert!(oops(5, 3, 1) == ());
}
