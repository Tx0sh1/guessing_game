use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..101);
    let mut lives = 5; // ask user how many lkives they want
    //add vector to store highscore for now

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        match guess.trim().parse::<u32>() {
            Ok(num) => {
                match num {
                    num if num < secret_number => {
                        lives = lives - 1;
                        if lives > 0 {
                            println!("Too small! You have {} lives left.", lives);
                        } else {
                            println!("You're out of lives, game over.");
                            break;
                        }
                    },
                    num if num > secret_number => {
                        lives = lives - 1;
                        if lives > 0 {
                            println!("Too big! You have {} lives left.", lives);
                        } else {
                            println!("You're out of lives, game over.");
                            break;
                        }
                    },
                    _ => {
                        println!("You win!");
                        println!("you guessed {}", num);
                        break;
                    }
                }
            }
            Err(_e) =>{
                println!("Please type a number!");
            }
        }
    
    }
}