use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum CharStatus {
    CORRECT,
    EXIST,
    WRONG,
}

fn check_guess(word: &str, guess: &str) -> Vec<CharStatus> {
    if guess.len() != word.len() {
        return vec![];
    }
    let mut status = vec![CharStatus::WRONG; guess.len()];
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for letter in word.chars() {
        *char_count.entry(letter).or_insert(0) += 1;
    }
    let word_chars: Vec<char> = word.chars().collect();
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
        match char_count.get_mut(&letter) {
            Some(cnt_ptr) => {
                if *cnt_ptr == 0 {
                    continue;
                } else {
                    status[idx] = CharStatus::EXIST;
                    *cnt_ptr -= 1;
                }
            }
            None => continue,
        }
    }

    return status;
}

fn main() {
    let word = "guess";
    let mut guesses_left = 6;
    let mut win = false;
    let mut buffer = String::new();
    while guesses_left > 0 && !win {
        print!("{} guess(es) left: ", guesses_left);
        io::stdout().flush().expect("failed to flush stdout");
        buffer.clear();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to readline");
        let guess = buffer.trim();
        let status = check_guess(word, guess);
        win = true;
        for letter_status in status.iter() {
            if *letter_status != CharStatus::CORRECT {
                win = false;
                break;
            }
        }
        println!("{:?}", status);
        guesses_left -= 1;
    }
    println!("{}", if win { "You won" } else { "You lost" });
}
