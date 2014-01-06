use std::os;


fn count_vowels(input: &str) -> int {
    let mut result = 0i;
    for c in input.iter() {
        match c {
            'a'|'e'|'i'|'o'|'u' => { result += 1; }
            _ => { }
        }
    }

    result
}


fn main() {
    let args = os::args();

    for arg in args.iter() {
        println(count_vowels(*arg).to_str());
    }
}
