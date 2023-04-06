use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let guess: String = read_one_line();

    println!("You guessed: {guess}");
}

fn read_one_line() -> String {
    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    return value;
}
