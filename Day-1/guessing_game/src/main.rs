use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your number");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse(){ Ok (num) => num, Err(_) => continue,}; //This is know as shadowing in Rust as we use the variable twice after decleaing it as a totally new variable
        //    |
        //    |
        //    |
        //    |
        //    |
        //    |                                                                
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            },
            Ordering::Less => println!("it's small!"),
            Ordering::Greater => println!("it's large!"),
        };

        println!("You guessed {guess}");
        //println!("You guessed {}", guess);
    }
}
