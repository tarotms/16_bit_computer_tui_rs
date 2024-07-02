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

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(20),
            Constraint::Percentage(20)
            ].as_ref())
        .split(size);

    /* Main panel */
    let text: Vec<Spans> = frame_buffer.buffer.iter().map(|line| Spans::from(Span::raw(line))).collect();

    let paragraph = Paragraph::new(text)
        .block(Block::default().title(" CPU ").borders(Borders::ALL));
    f.render_widget(paragraph, chunks[0]);
    
    /* Register LED panel */
    let register_texts = format_registers_with_leds(&frame_buffer.registers_led);

    let register_paragraph = Paragraph::new(register_texts)
        .block(Block::default().title(" Registers ").borders(Borders::ALL));

    f.render_widget(register_paragraph, chunks[1]);

    /* Settings panel */
    let settings_texts: Vec<Spans> = frame_buffer.settings.iter().map(|setting| {
        let description = setting.description();
        let value = setting.value();
        Spans::from(format!("{}: {}", description, value))
    }).collect();

    let settings_paragraph = Paragraph::new(settings_texts)
        .block(Block::default().title("Settings").borders(Borders::ALL));

    f.render_widget(settings_paragraph, chunks[2]);

}

fn format_registers_with_leds(registers: &[u16; 8]) -> Vec<Spans> {
    let mut register_texts = Vec::new();

    for (i, &register_value) in registers.iter().enumerate() {
        let leds: String = (0..16)
            .map(|bit| {
                if (register_value & (1 << (15 - bit))) != 0 {
                    "●"
                } else {
                    "○"
                }
            })
            .collect();

        let register_text = format!("R{}: \n{}", i, leds);
        register_texts.push(Spans::from(register_text));
    }

    register_texts
}