mod dict_loader;
mod game_core;
mod word_picker;

use std::io::{self, Write};

use crate::game_core::{CharStatus, GameState};
use crate::word_picker::RandomPicker;

fn dump_guess_result(guess: &str, result: Vec<CharStatus>) {
    println!("{guess}");
    println!("{:?}", result);
}

fn main() {
    let dict = dict_loader::load_default_dict();
    let picker = RandomPicker::new(dict.iter().cloned().collect());
    let word_len = 5;
    let guesses = 6;

    for answer in picker {
        println!("==NEW ROUND==");
        let mut game = match GameState::new_game_with_dict(guesses, word_len, &dict, answer) {
            Ok(game) => game,
            Err(e) => {
                println!("failed to init game: {e}");
                return;
            }
        };
        let mut buffer = String::new();
        while game.guesses_left > 0 && !game.won {
            print!("{} guess(es) left: ", game.guesses_left);
            io::stdout().flush().expect("failed to flush stdout");
            buffer.clear();
            io::stdin()
                .read_line(&mut buffer)
                .expect("failed to readline");
            let guess = buffer.trim();
            match game.make_guess(guess) {
                Ok(result) => dump_guess_result(guess, result),
                Err(e) => println!("failed: {e}"),
            }
        }
        println!("{}", if game.won { "You won" } else { "You lost" });
    }
}
