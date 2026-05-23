use std::collections::HashMap;

use crate::game_core::CharStatus;
use crate::game_core::GameState;

impl<'a> GameState<'a> {
    pub fn make_guess(&mut self, guess: &str) -> Result<Vec<CharStatus>, String> {
        if self.guesses_left == 0 {
            return Err(String::from("out of guess"));
        }

        if let Err(e) = self.check_valid_guess(guess) {
            return Err(e);
        }

        self.guesses_left -= 1;
        if self.answer.eq(guess) {
            self.won = true;
            return Ok(vec![CharStatus::CORRECT; self.word_len]);
        }

        return Ok(check_guess(&self.answer, guess));
    }

    fn check_valid_guess(&self, guess: &str) -> Result<bool, String> {
        if guess.len() != self.word_len {
            return Err(format!(
                "invalid guess len, got {}, expected {}",
                guess.len(),
                self.word_len,
            ));
        }

        if self.use_dict && !self.dictionary.contains(guess) {
            return Err(String::from("guess not present in dictionary"));
        }

        return Ok(true);
    }
}

fn check_guess(answer: &str, guess: &str) -> Vec<CharStatus> {
    let mut status = vec![CharStatus::WRONG; guess.len()];
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for letter in answer.chars() {
        *char_count.entry(letter).or_insert(0) += 1;
    }
    let word_chars: Vec<char> = answer.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    // check for CORRECT letters first
    for (idx, letter) in guess_chars.iter().enumerate() {
        if *letter == word_chars[idx] {
            status[idx] = CharStatus::CORRECT;
            char_count
                .entry(*letter)
                .and_modify(|cnt_ptr| *cnt_ptr -= 1);
        }
    }

    // check for EXIST
    for (idx, letter) in guess_chars.iter().enumerate() {
        if status[idx] == CharStatus::CORRECT {
            continue;
        }
        if let Some(cnt_ptr) = char_count.get_mut(&letter) {
            if *cnt_ptr == 0 {
                continue;
            } else {
                status[idx] = CharStatus::EXIST;
                *cnt_ptr -= 1;
            }
        }
    }

    return status;
}
