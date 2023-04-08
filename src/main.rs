// extern crate rand;

use rand::Rng;
use std::env;
use std::io;

fn main() {
    // println!("Welcome to our Guessing game");
    // println!("Your task is to find the magic number between the range of the two numbers below that you will defined");

    println!("Choose the start of the range number");
    let mut range_start = String::new();
    io::stdin()
        .read_line(&mut range_start)
        .unwrap();
    
    let trimmed: i32 = range_start.trim().parse().unwrap();
    println!("hello {}", trimmed);
}
