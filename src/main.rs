mod dict_loader;
mod game_core;
mod word_picker;
mod tui;

use std::io::{self};

use crate::tui::Tui;

fn main() -> io::Result<()>{
    ratatui::run(|terminal| Tui::default().run(terminal))
}
