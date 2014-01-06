use std::os;


fn reverse(input: &str) -> ~str {
    let mut result = ~"";
    for c in input.rev_iter() {
        result.push_char(c);
    }
    result
}


#[test]
fn test_reverse() {
    assert!(reverse("racecar") == ~"racecar");
    assert!(reverse("hello") == ~"olleh");
    assert!(reverse("12345") == ~"54321");
}



fn main() {
    let args = os::args();

    for arg in args.iter() {
        println(reverse(*arg));
    }
}
