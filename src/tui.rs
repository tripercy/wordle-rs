mod game;
mod menu;

use ratatui::DefaultTerminal;
use std::{collections::HashSet, io};

use crate::{
    dict_loader,
    game_core::GameState,
    tui::{game::Game, menu::Menu},
    word_picker,
};

#[derive(PartialEq, Clone, Copy)]
enum AppState {
    MENU,
    GAME,
    QUIT,
}

pub struct Tui<I>
where
    I: Iterator<Item = String>,
{
    dictionary: HashSet<String>,
    word_picker: I,
}

impl Default for Tui<word_picker::RandomPicker> {
    fn default() -> Self {
        let dictionary = dict_loader::load_default_dict();
        Self {
            word_picker: word_picker::RandomPicker::new(dictionary.iter().cloned().collect()),
            dictionary,
        }
    }
}

impl<T: Iterator<Item = String>> Tui<T> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut current_state = AppState::MENU;
        while current_state != AppState::QUIT {
            current_state = match current_state {
                AppState::MENU => Menu::default().run(terminal)?,
                AppState::GAME => Game::new(
                    GameState::new_game_with_dict(
                        6,
                        5,
                        &self.dictionary,
                        self.word_picker.next().unwrap(),
                    )
                    .expect("failed to create game state"),
                )
                .run(terminal)?,
                _ => AppState::QUIT,
            }
        }
        Ok(())
    }
}
