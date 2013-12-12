use std::io;

fn main() {
    println("INPUT: ");
    let input = io::stdin().read_line();

    println("YOU TYPED: ");
    println(input);
}
