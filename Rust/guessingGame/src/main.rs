extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let mut input = String::new();

    println!("Your number:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let key = rand::thread_rng().gen_range(1..100);
    print!("you guessed {} and the correct answer is {}\n", input, key)
}
