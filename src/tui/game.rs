use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Paragraph},
};
use std::io;

use crate::{
    game_core::{CharStatus, GameState},
    tui::{AppState, custom_widgets::blocky_text::BlockyText},
};

/*
 * Posisble transitions:
 * INPUT -> QUITTING
 * INPUT -> FINISH
 * QUITTING -> INPUT
 * QUITTING -> running.false
 * FINISH -> running.false
 * */
enum InputState {
    Input,
    Quitting,
    Finish,
}

pub struct Game<'a> {
    running: bool,
    input_state: InputState,
    game_state: GameState<'a>,
    next_screen: AppState,
    input_buffer: String,

    guess_area_height: u16,
    guess_width: u16,
    guesses: Vec<BlockyText<'a>>,

    noti_title: String,
    noti_detail: String,
}

impl<'a> Game<'a> {
    pub fn new(game_state: GameState<'a>) -> Game<'a> {
        let guess_area_height = 3 * 6 + 2; // TODO: replace with config number of guesses
        let guess_width = 5 * 5 + 2; // TODO: replace with word len
        Game {
            running: true,
            input_state: InputState::Input,
            next_screen: AppState::Menu,
            guesses: vec![],
            game_state,
            guess_area_height,
            guess_width,
            input_buffer: String::new(),
            noti_title: String::new(),
            noti_detail: String::new(),
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<AppState> {
        while self.running {
            terminal.draw(|f| self.render(f))?;
            self.handle_input()?;
        }
        Ok(self.next_screen)
    }
}

impl<'a> Game<'a> {
    fn render(&self, frame: &mut Frame) {
        let rows = Layout::vertical([
            Constraint::Length(self.guess_area_height),
            Constraint::Length(5),
            Constraint::Length(4),
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
        self.render_notification(
            frame,
            rows[2].centered_horizontally(Constraint::Ratio(1, 3)),
        );
        self.render_keyboard_area(
            frame,
            rows[3].centered_horizontally(Constraint::Ratio(1, 3)),
        );
    }

    fn render_guess_area(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title(format!(
                "Guesses ({}/{})",
                self.game_state.guesses_max - self.game_state.guesses_left,
                self.game_state.guesses_max
            ));
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
        let block = Block::bordered().title("Current Guess");
        let inner = block.inner(area);

        frame.render_widget(BlockyText::new(self.input_buffer.chars(), vec![]), inner);
        frame.render_widget(block, area);
    }

    fn render_notification(&self, frame: &mut Frame, area: Rect) {
        let content = Paragraph::new(vec![
            Line::styled(&self.noti_title, Style::new().bold()),
            Line::styled(&self.noti_detail, Style::new()),
        ])
        .block(Block::bordered().border_type(BorderType::Double));

        frame.render_widget(content, area);
    }

    fn render_keyboard_area(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::bordered().title("Keyboard"), area);
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match self.input_state {
                InputState::Input => self.handle_input_input_state(key.code),
                InputState::Quitting => self.handle_input_quitting_state(key.code),
                InputState::Finish => self.handle_input_finish_state(key.code),
            }
        }

        Ok(())
    }

    fn handle_input_input_state(&mut self, key_code: KeyCode) {
        self.clear_noti();
        match key_code {
            KeyCode::Esc => self.enter_quitting_state(),
            KeyCode::Backspace => self.del_char(),
            KeyCode::Enter => self.make_guess(),
            KeyCode::Char(c) => self.add_char(c),
            _ => {}
        }
    }

    fn handle_input_quitting_state(&mut self, key_code: KeyCode) {
        if key_code.is_esc() {
            self.quit();
        } else {
            self.exit_quitting_state();
        }
    }

    fn handle_input_finish_state(&mut self, key_code: KeyCode) {
        if key_code.is_esc() || key_code.is_char('q') {
            self.next_screen = AppState::Menu;
        } else {
            self.next_screen = AppState::Game;
        }
        self.quit();
    }

    fn enter_quitting_state(&mut self) {
        self.input_state = InputState::Quitting;
        self.set_noti(
            "Quitting?",
            "<Esc> to quit to menu, any key to continue input.",
        );
    }

    fn exit_quitting_state(&mut self) {
        self.input_state = InputState::Input;
        self.clear_noti();
    }

    fn add_char(&mut self, c: char) {
        // TODO: replace with game word len
        if self.input_buffer.len() == 5 {
            self.set_noti("Word len limit hit", "Stop typing already man");
        } else {
            self.input_buffer.push(c);
        }
    }

    fn del_char(&mut self) {
        match self.input_buffer.pop() {
            Some(_) => {}
            None => self.set_noti("Nothing to delete", "What are you even trying to do?"),
        }
    }

    fn make_guess(&mut self) {
        let guess = self.input_buffer.to_lowercase();
        // TODO: replace with game word len
        if guess.len() != 5 {
            self.set_noti(
                "It's too short",
                &format!("{} is too short :(", guess.len()),
            );
            return;
        }
        match self.game_state.make_guess(&guess) {
            Ok(result) => {
                self.add_guess(&guess, result);
                self.input_buffer.clear();
                self.check_game_end();
            }
            Err(msg) => self.set_noti("Couldn't submit", &msg),
        }
    }

    fn add_guess(&mut self, guess: &str, result: Vec<CharStatus>) {
        let styles: Vec<Style> = result
            .iter()
            .map(|status| Self::map_char_status_to_style(*status))
            .collect();
        self.guesses.push(BlockyText::new(guess.chars(), styles));
    }

    fn check_game_end(&mut self) {
        if self.game_state.won {
            self.input_state = InputState::Finish;
            self.set_noti(
                "You won!!",
                "<esc>/<q> to quit to menu, any other key to start anew",
            );
        } else if self.game_state.guesses_left == 0 {
            self.input_state = InputState::Finish;
            self.set_noti(
                &format!(
                    "You lost, nerd! The word was {}",
                    self.game_state.get_answer().unwrap()
                ),
                "<esc>/<q> to run away, any key to lose again.",
            );
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn map_char_status_to_style(status: CharStatus) -> Style {
        match status {
            CharStatus::Correct => Style::new().green(),
            CharStatus::Exist => Style::new().yellow(),
            CharStatus::Wrong => Style::new().gray(),
        }
    }

    fn set_noti(&mut self, title: &str, detail: &str) {
        self.clear_noti();
        self.noti_title.push_str(title);
        self.noti_detail.push_str(detail);
    }

    fn clear_noti(&mut self) {
        self.noti_title.clear();
        self.noti_detail.clear();
    }
}
