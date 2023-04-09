// extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to our Guessing game");
    println!("Your task is to find the magic number between the range of the two numbers below that you will defined \n");

    println!("Choose the start of the range number: ");
    let mut input_range_start = String::new();
    io::stdin()
        .read_line(&mut input_range_start)
        .unwrap();
    
    let range_start: i32 = input_range_start.trim().parse().unwrap();
    

    println!("Choose the end of the range number: ");
    let mut input_range_stop = String::new();
    io::stdin()
        .read_line(&mut input_range_stop)
        .unwrap();
    
    let range_stop: i32 = input_range_stop.trim().parse().unwrap();

    println!("The numbers you selected are {} and {} \n", range_start, range_stop);

    let random_number = rand::thread_rng().gen_range(range_start..range_stop);
    let is_magic_number_found = false;

    while !is_magic_number_found {
        println!("Guess what the magic number is:");
        println!("Choose a number between {} and {}", range_start, range_stop);
        let mut input_guessed_number = String::new();
        io::stdin()
            .read_line(&mut input_guessed_number)
            .unwrap();
        
        let guessed_number: i32 = input_guessed_number.trim().parse().unwrap();

        if guessed_number > range_start &&
           guessed_number < range_stop &&
           random_number != guessed_number {
            println!("\n");
            println!("wrong number: ");
            println!("The magic number was: {}", random_number);
            println!("Please try again \n");
        } else if guessed_number > random_number {
            println!("The number selected is too high and out of the range: ");
            println!("Please try again \n");
        } else if guessed_number < random_number {
            println!("The number selected is too low and out of the range: ");
            println!("Please try again \n");
        } else {
            println!("YOU WON!!!!!, the number is {}", random_number);
            break
        }
    }
}
