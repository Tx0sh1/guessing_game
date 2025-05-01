use std::io;
//use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("you guessed {}", guess);

    let secret_number = rand::random_range(1..101);

    if guess < secret_number {
        println!("Too small!");
    } else if guess > secret_number {
        println!("Too big!");
    } else {
        println!("You win!");
    }
    println!("The secret number was: {}", secret_number);
}