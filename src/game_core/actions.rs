use std::cmp;
use std::collections::HashMap;

use crate::game_core::CharStatus;
use crate::game_core::GameState;

impl<'a> GameState<'a> {
    pub fn make_guess(&mut self, guess: &str) -> Result<Vec<CharStatus>, String> {
        if self.won {
            return Err(String::from("already won"));
        }

        if self.guesses_left == 0 {
            return Err(String::from("out of guess"));
        }

        self.check_valid_guess(guess)?;

        self.guesses_left -= 1;
        if self.answer.eq(guess) {
            self.won = true;
            return Ok(vec![CharStatus::Correct; self.word_len]);
        }

        let result = check_guess(&self.answer, guess);
        self.update_char_status(guess, &result);

        Ok(result)
    }

    pub fn get_answer(&'a self) -> Result<&'a str, &'a str> {
        if !self.won && self.guesses_left > 0 {
            return Err("game must be finished before answer can be accessed");
        }
        Ok(&self.answer)
    }

    fn check_valid_guess(&self, guess: &str) -> Result<(), String> {
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

        Ok(())
    }

    fn update_char_status(&mut self, guess: &str, result: &Vec<CharStatus>) {
        for (i, ch) in guess.chars().enumerate() {
            self.char_status
                .entry(ch)
                .and_modify(|status| *status = cmp::min(*status, result[i]))
                .or_insert(result[i]);
        }
    }
}

fn check_guess(answer: &str, guess: &str) -> Vec<CharStatus> {
    let mut status = vec![CharStatus::Wrong; guess.len()];
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for letter in answer.chars() {
        *char_count.entry(letter).or_insert(0) += 1;
    }
    let word_chars: Vec<char> = answer.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    // check for CORRECT letters first
    for (idx, letter) in guess_chars.iter().enumerate() {
        if *letter == word_chars[idx] {
            status[idx] = CharStatus::Correct;
            char_count
                .entry(*letter)
                .and_modify(|cnt_ptr| *cnt_ptr -= 1);
        }
    }

    // check for EXIST
    for (idx, letter) in guess_chars.iter().enumerate() {
        if status[idx] == CharStatus::Correct {
            continue;
        }
        if let Some(cnt_ptr) = char_count.get_mut(letter) {
            if *cnt_ptr == 0 {
                continue;
            } else {
                status[idx] = CharStatus::Exist;
                *cnt_ptr -= 1;
            }
        }
    }

    status
}
