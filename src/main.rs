/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 */

use crate::frame_buffer::FrameBuffer;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};


mod utils;
mod program_count;
mod ram;
mod cpu;
mod gate;
mod ui;
mod frame_buffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    /* Initialization terminal */
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let welcome = utils::welcome() + "Press 's' to startup\n";

    let mut frame_buffer = FrameBuffer::default();
    frame_buffer.push_msg(welcome);

    loop {
        terminal.draw(|f| {
                ui::ui(f, &frame_buffer)})?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('s') {
                let mut cpu = cpu::CPU::new();
                cpu::startup(&mut cpu, &mut frame_buffer, &mut terminal)?;
                break;
            }
        }
    }

    /* Restore terminal */
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    
    terminal.show_cursor()?;

    Ok(())
}

