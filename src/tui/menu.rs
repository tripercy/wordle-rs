use std::io;

use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame, layout::Constraint, style::{Color, Style}, widgets::{Block, List, ListState}
};

use crate::tui::AppState;

pub struct Menu {
    running: bool,
    select_index: usize,
    selected_item: AppState,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            running: true,
            select_index: 0,
            selected_item: Self::MENU_ITEMS[0],
        }
    }
}

impl Menu {
    const MENU_ITEMS: &'static [AppState] = &[AppState::GAME, AppState::QUIT];

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<AppState> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_input()?;
        }

        Ok(self.selected_item)
    }
}

impl Menu {
    fn render(&mut self, frame: &mut Frame) {
        // inefficient, but there are only 2 items
        let items: Vec<&str> = Self::MENU_ITEMS
            .iter()
            .map(|s| Self::map_menu_item_to_str(*s))
            .collect();

        let list = List::new(items)
            .style(Color::LightGreen)
            .highlight_style(Style::default().fg(Color::Cyan))
            .highlight_symbol("> ");

        frame.render_stateful_widget(
            list,
            frame.area().centered(
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ),
            &mut ListState::default().with_selected(Some(self.select_index)),
        );
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.update_selection(1),
                KeyCode::Char('k') | KeyCode::Up => self.update_selection(-1),
                KeyCode::Enter | KeyCode::Char(' ') => self.select_item(),
                KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                _ => {}
            }
        }

        Ok(())
    }

    fn update_selection(&mut self, direction: isize) {
        let menu_len = Self::MENU_ITEMS.len();
        self.select_index =
            ((self.select_index + menu_len).wrapping_add_signed(direction)) % menu_len;
    }

    fn select_item(&mut self) {
        self.running = false;
        self.selected_item = Self::MENU_ITEMS[self.select_index];
    }

    fn quit(&mut self) {
        self.running = false;
        self.selected_item = AppState::QUIT;
    }

    fn map_menu_item_to_str(state: AppState) -> &'static str {
        match state {
            AppState::GAME => "Start",
            AppState::QUIT => "Quit",
            AppState::MENU => "HOW", // no, fr, how?
        }
    }
}
