use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Block,
};
use std::io;

use crate::{game_core::GameState, tui::AppState};

pub struct Game<'a> {
    game_state: GameState<'a>,

    guesses: Vec<Line<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(game_state: GameState<'a>) -> Game<'a> {
        return Game {
            game_state,
            guesses: vec![],
        };
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<AppState> {
        while self.game_state.guesses_left > 0 && !self.game_state.won {
            terminal.draw(|f| self.render(f))?;
            self.handle_input()?;
        }
        return Ok(AppState::MENU);
    }
}

impl<'a> Game<'a> {
    fn render(&self, frame: &mut Frame) {
        let guess_area_height = 3 * 6 + 2; // TODO: replace with config number of guesses
        let guess_width = 3 * 5 + 2; // TODO: replace with word len
        let rows = Layout::vertical([
            Constraint::Length(guess_area_height),
            Constraint::Length(5),
            Constraint::Fill(1),
        ])
        .split(frame.area());

        self.render_guess_area(
            frame,
            rows[0].centered_horizontally(Constraint::Length(guess_width)),
        );
        self.render_input_area(
            frame,
            rows[1].centered_horizontally(Constraint::Length(guess_width)),
        );
        self.render_keyboard_area(
            frame,
            rows[2].centered_horizontally(Constraint::Ratio(1, 3)),
        );
    }

    fn render_guess_area(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::bordered().title("Guesses"), area);
    }

    fn render_input_area(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::bordered().title("Current Guess"), area);
    }

    fn render_keyboard_area(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::bordered().title("Keyboard"), area);
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Esc => self.quit(),
                _ => {}
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.game_state.won = true;
    }
}
