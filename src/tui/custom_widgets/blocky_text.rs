use std::{cmp::min, str::Chars};

use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Padding, Widget},
};

#[derive(Clone)]
pub struct BlockyText<'a> {
    len: usize,
    lines: Vec<Line<'a>>,
    blocks: Vec<Block<'a>>,
    cell_constraints: Vec<Constraint>,
}

impl<'a> Widget for BlockyText<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let render_line = Layout::vertical([Constraint::Length(3)]).split(area)[0];

        let cells = Layout::horizontal(self.cell_constraints.iter()).split(render_line);
        for (i, cell) in cells.iter().enumerate() {
            self.blocks[i].clone().render(*cell, buf);
            let inner = self.blocks[i].inner(*cell);
            self.lines[i].clone().render(inner, buf);
        }
    }
}

impl<'a> BlockyText<'a> {
    pub fn new(text: Chars, styles: Vec<Style>) -> BlockyText<'a> {
        let style_len = styles.len();
        let mut j = 0;

        let mut lines: Vec<Line<'a>> = Vec::new();
        let mut blocks: Vec<Block<'a>> = Vec::new();
        for ch in text {
            lines.push(
                Line::from(ch.to_string().to_uppercase())
                    .style(styles.get(j).unwrap_or(&Style::new()).to_owned()),
            );
            blocks.push(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .padding(Padding::horizontal(1))
                    .style(styles.get(j).unwrap_or(&Style::new()).to_owned()),
            );

            j = min(j + 1, style_len);
        }
        let mut cell_constraints: Vec<Constraint> = Vec::new();
        for _ in 0..lines.len() {
            cell_constraints.push(Constraint::Length(5));
        }

        BlockyText {
            len: lines.len() * 5,
            lines,
            blocks,
            cell_constraints,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
