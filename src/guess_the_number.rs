use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_the_number() {
    println!("Lets play Guess the Number:-");
    println!("Number should be between 1 to 100!!");
    let min_num = 1;
    let max_num = 100;
    let num_to_guess = rand::thread_rng().gen_range(min_num..=max_num);
    println!("number_to_guess:-{num_to_guess}");
    loop {
        println!("Enter your number:-");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Enter an input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("my guess:-{guess}");
        match guess.cmp(&num_to_guess) {
            Ordering::Equal => {
                println!("You guess correctly!!!");
                break;
            }
            Ordering::Greater => println!("Your number is greater then number to guess"),
            Ordering::Less => println!("Your number is lesser then number to guess"),
        }
    }
}
