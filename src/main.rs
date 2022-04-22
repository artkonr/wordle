extern crate core;

mod bank;
mod rules;

use std::io::stdin;
use std::process::exit;
use colored::Colorize;
use crate::bank::{Dictionary, StaticDict};

const ATTEMPT_COUNT: u8 = 6;

fn main() {

    println!("Welcome to Wordle!");

    let dict = StaticDict;
    let secret = dict.generate();

    println!("_ _ _ _ _");

    for attempt_n in 0..ATTEMPT_COUNT {

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let guess = String::from(input.trim_end());

        let result = secret.try_match(&guess);

        if result.full_match() {
            println!(
                "{} {}",
                "You won!".green(),
                format!("You needed {} attempts", attempt_n).normal()
            );
            exit(0)
        } else {
            result.print_result_for(&guess);
        }

    }

    println!("{} The word was '{}'", "You lost :(".red(), secret.reveal())

}
