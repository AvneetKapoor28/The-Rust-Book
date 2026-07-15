use std::io;

fn main() {
    println!("GUESS THE NUMBER!");
    println!("Please enter you guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
