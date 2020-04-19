use std::io;

fn main() {
    println!("Guessing numbers game");
    println!("Please enter your guess. ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
