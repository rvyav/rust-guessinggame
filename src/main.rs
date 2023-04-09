// extern crate rand;

use rand::Rng;
use rand::random;
use std::env;
use std::io;

fn main() {
    // println!("Welcome to our Guessing game");
    // println!("Your task is to find the magic number between the range of the two numbers below that you will defined");

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

    println!("The numbers you selected are {} and {}", range_start, range_stop);

    let random_number = rand::thread_rng().gen_range(range_start..range_stop);
    println!("Guessing number is {}", random_number);
}
