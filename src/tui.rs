mod menu;

use std::io;
use ratatui::DefaultTerminal;

use crate::tui::menu::Menu;

#[derive(PartialEq, Clone, Copy)]
enum AppState {
    MENU,
    GAME,
    QUIT,
}

#[derive(Default)]
pub struct Tui {}

impl Tui {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut current_state = AppState::MENU;
        while current_state != AppState::QUIT {
            current_state = match current_state {
                AppState::MENU => Menu::default().run(terminal)?,
                // AppState::GAME => todo!(),
                _ => AppState::QUIT,
            }
        }
        Ok(())
    }
}
