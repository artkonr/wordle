
mod bank;
mod game;

use std::process::exit;
use crate::bank::{Dictionary, StaticDict};
use crate::game::start_game_loop;

const ATTEMPT_COUNT: u8 = 6;

fn main() {

    println!("Welcome to Wordle!");

    let dict = StaticDict;
    let secret = dict.generate();

    println!("_ _ _ _ _");

    match start_game_loop(&secret) {
        Ok(_) => exit(0),
        Err(e) => {
            println!("{}", e);
        }
    }

}