
fn match_number(input: int) {
    match input {
        0     => println!("({:2d}) zero", input),
        1 | 2 => println!("({:2d}) one or two", input),
        3..10 => println!("({:2d}) three to ten", input),
        _     => println!("({:2d}) something else", input)
    }
}

fn main() {
    let mut num = 0;
    while num < 12 {
        match_number(num);
        num += 1;
    }
}
