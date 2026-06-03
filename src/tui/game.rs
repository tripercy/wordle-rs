use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType},
};
use std::io;

use crate::{
    game_core::{CharStatus, GameState},
    tui::{AppState, custom_widgets::blocky_text::BlockyText},
};

pub struct Game<'a> {
    game_state: GameState<'a>,
    guess_area_height: u16,
    guess_width: u16,
    guesses: Vec<BlockyText<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(game_state: GameState<'a>) -> Game<'a> {
        let guess_area_height = 3 * 6 + 2; // TODO: replace with config number of guesses
        let guess_width = 5 * 5 + 2; // TODO: replace with word len
        return Game {
            game_state,
            guess_area_height,
            guess_width,
            guesses: vec![],
        };
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<AppState> {
        self.make_guess("tests");
        self.make_guess("adieu");
        self.make_guess("guess");
        while self.game_state.guesses_left > 0 && !self.game_state.won {
            terminal.draw(|f| self.render(f))?;
            self.handle_input()?;
        }
        return Ok(AppState::MENU);
    }
}

impl<'a> Game<'a> {
    fn render(&self, frame: &mut Frame) {
        let rows = Layout::vertical([
            Constraint::Length(self.guess_area_height),
            Constraint::Length(5),
            Constraint::Fill(1),
        ])
        .split(frame.area());

        self.render_guess_area(
            frame,
            rows[0].centered_horizontally(Constraint::Ratio(1, 4)),
        );
        self.render_input_area(
            frame,
            rows[1].centered_horizontally(Constraint::Length(self.guess_width)),
        );
        self.render_keyboard_area(
            frame,
            rows[2].centered_horizontally(Constraint::Ratio(1, 3)),
        );
    }

    fn render_guess_area(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title("Guesses");
        let inner = block.inner(area);
        // Split area into max guesses, currently hard code 6 guesses
        let mut constraints: Vec<Constraint> = Vec::new();
        for _ in 0..6 {
            constraints.push(Constraint::Length(3));
        }
        let rows = Layout::vertical(constraints).split(inner);

        // render
        frame.render_widget(block, area);
        for (i, line) in self.guesses.iter().enumerate() {
            frame.render_widget(
                line.clone(),
                rows[i].centered_horizontally(Constraint::Length(self.guess_width)),
            );
        }
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

    fn make_guess(&mut self, guess: &str) {
        match self.game_state.make_guess(guess) {
            Ok(result) => self.add_guess(guess, result),
            Err(_) => todo!(),
        }
    }

    fn add_guess(&mut self, guess: &str, result: Vec<CharStatus>) {
        let styles: Vec<Style> = result
            .iter()
            .map(|status| Self::map_char_status_to_style(*status))
            .collect();
        self.guesses.push(BlockyText::new(guess.chars(), styles));
    }

    fn quit(&mut self) {
        self.game_state.won = true;
    }

    fn map_char_status_to_style(status: CharStatus) -> Style {
        match status {
            CharStatus::CORRECT => Style::new().green(),
            CharStatus::EXIST => Style::new().yellow(),
            CharStatus::WRONG => Style::new().gray(),
        }
    }
}
