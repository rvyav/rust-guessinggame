extern crate rand;

use rand::Rng;
use std::io;

use guessing_game::convert_string_input_to_integer;


fn main() {
    println!("Welcome to our Guessing game");
    println!("Your task is to guess the magic number between the range of the two numbers below that you will defined \n");

    println!("Choose the start of the range number: ");
    let input_range_start = String::new();
    let range_start = convert_string_input_to_integer(input_range_start);

    println!("Choose the end of the range number: ");
    let input_range_stop = String::new();
    let range_stop = convert_string_input_to_integer(input_range_stop);

    println!("The numbers you selected are {} and {} \n", range_start, range_stop);

    let random_number = rand::thread_rng().gen_range(range_start..range_stop);
    let mut is_magic_number_found = false;

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
            is_magic_number_found = true;
        }
    }
}


