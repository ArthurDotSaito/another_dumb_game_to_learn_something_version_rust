use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Bro, guess a number!");

    let secret = rand::thread_rng().gen_range(1..=101);

    loop {
        println!("Input your guess: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
