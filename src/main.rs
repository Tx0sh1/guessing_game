use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..101);
    let mut lives = 5;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("you guessed {}", guess);

        if guess < secret_number {
            println!("Too small!");
            lives = lives - 1;
            println!("you have {} lives remaining", lives);
                if lives == 0{
                    println!("you are out of lives, game over");
                    break;
                }
        } else if guess > secret_number {
            println!("Too big!");
            lives = lives - 1;
            println!("you have {} lives remaining", lives);
                if lives == 0{
                    println!("you are out of lives, game over");
                    break;
                }
        } else {
            println!("You win!");
            break;
        }
        }
    
}