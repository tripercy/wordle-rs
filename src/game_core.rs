mod actions;
mod char_status;
mod game;

use std::collections::HashSet;

pub use char_status::CharStatus;

#[derive(Debug)]
pub struct GameState<'a> {
    pub guesses_left: usize,
    pub won: bool,
    guesses_max: usize,
    word_len: usize,
    use_dict: bool,
    dictionary: &'a HashSet<String>,
    answer: String,
}
