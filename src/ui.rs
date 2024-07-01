use crate::frame_buffer::FrameBuffer;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, frame_buffer: &FrameBuffer) {
    let size = f.size();

    let block = Block::default()
        .title("TUI Example")
        .borders(Borders::ALL);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let text: Vec<Spans> = frame_buffer.buffer.iter().map(|line| Spans::from(Span::raw(line))).collect();

    let paragraph = Paragraph::new(text)
        .block(Block::default().title(" CPU ").borders(Borders::ALL));
    f.render_widget(paragraph, chunks[0]);
}