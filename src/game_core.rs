mod actions;
mod char_status;
mod game;

use std::collections::{HashMap, HashSet};

pub use char_status::CharStatus;

#[derive(Debug)]
pub struct GameState<'a> {
    pub guesses_left: usize,
    pub won: bool,
    pub guesses_max: usize,
    pub char_status: HashMap<char, CharStatus>,
    word_len: usize,
    use_dict: bool,
    dictionary: &'a HashSet<String>,
    answer: String,
}
