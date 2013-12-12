
fn is_three(num: int) -> bool {
    num % 3 == 0
}

#[test]
fn test_is_three() {
    assert!(!is_three(1));
    assert!(is_three(3));
}


fn is_five(num: int) -> bool {
    num % 5 == 0
}


fn main() {
    for num in range(1, 101) {
        print!("{:3d}: ", num);
        if is_three(num) {
            print("Fizz");
        }
        if is_five(num) {
            print("Buzz");
        }
        print("\n");
    }
}
