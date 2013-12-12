use std::io;
use std::rand;

/// return random number from 1 to 100
fn generate_secret_number() -> int {
    (rand::random::<int>() % 100).abs() + 1
}


fn process_guess(answer: int, guess: int) {
    if guess < answer {
        println!("{} is too low", guess);
    } else if guess > answer {
        println!("{} is too high", guess);
    } else {
        println!("YOU GOT IT!");
    }
}


fn main() {
    println("--- NUMBER GAME ---");
    println("");
    println("Guess a number from 1-100: ");

    let answer = generate_secret_number();

    let mut tries_remaining = 5;
    while tries_remaining > 0 {
        print!("{} tries remaining: ", tries_remaining);
        let guess_str = io::stdin().read_line();

        match from_str::<int>(guess_str) {
            Some(guess) => {
                process_guess(answer, guess);
                if answer == guess { break; }
                tries_remaining -= 1;
            }
            None => {
                println("invalid input, enter a number");
            }
        }

    }
}
