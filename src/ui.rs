use std::cell::RefCell;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::Rc;
use crate::{utils, Logger};

pub fn ui<B: Backend>(f: &mut Frame<B>, log: Rc<RefCell<Logger>>) {
    let size = f.size();

    let blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(size);

    let block1 = Block::default()
        .title(utils::format_title("block 1"))
        .borders(Borders::ALL);
    f.render_widget(block1, blocks[0]);

    let log_clone = log.borrow().buffer.clone();
    let log_text: Vec<Spans> = log_clone.iter().map(|line| Spans::from(Span::raw(line))).collect();

    let paragraph = Paragraph::new(log_text)
        .block(Block::default().title(utils::format_title("Block 2")).borders(Borders::ALL));
    f.render_widget(paragraph, blocks[1]);
}