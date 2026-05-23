use crate::game_core::GameState;
use std::collections::HashSet;

impl GameState {
    pub fn new_game_with_dict(
        guesses_max: usize,
        word_len: usize,
        valid_words: HashSet<String>,
        answer: String,
    ) -> Result<GameState, String> {
        let game_state = GameState {
            guesses_max,
            guesses_left: guesses_max,
            won: false,
            word_len,
            use_dict: true,
            dictionary: valid_words,
            answer,
        };

        if let Err(err_msg) = game_state.check_init_state_valid() {
            return Err(err_msg);
        }

        return Ok(game_state);
    }

    fn check_init_state_valid(&self) -> Result<bool, String> {
        if self.use_dict {
            for word in &(self.dictionary) {
                if word.len() != self.word_len {
                    return Err(format!(
                        "invalid word in dict: {word}, len: {}, expected: {}",
                        word.len(),
                        self.word_len
                    ));
                }
            }

            if !self.dictionary.contains(&self.answer) {
                return Err(String::from("answer not found in dict"));
            }
        } else {
            if self.answer.len() != self.word_len {
                return Err(format!(
                    "invalid answer len:  len: {}, expected: {}",
                    self.answer.len(),
                    self.word_len
                ));
            }
        }
        return Ok(true);
    }
}
