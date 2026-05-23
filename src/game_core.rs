mod actions;
mod char_status;
mod game;

use std::collections::HashSet;

pub use char_status::CharStatus;

#[derive(Debug)]
pub struct GameState {
    pub guesses_left: usize,
    pub won: bool,
    guesses_max: usize,
    word_len: usize,
    use_dict: bool,
    dictionary: HashSet<String>,
    answer: String,
}
