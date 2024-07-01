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
mod assembly;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    /* Initialization terminal */
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut welcome = utils::welcome();
    welcome += "Press 1 -> The sum of numbers from 0 to 100\n";

    welcome += "Press Q -> Quit\n";

    let mut frame_buffer = FrameBuffer::default();
    frame_buffer.push_msg(welcome.clone());

    loop {
        terminal.draw(|f| {
                ui::ui(f, &frame_buffer)})?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('1') {
                let mut cpu = cpu::CPU::new();
                
                cpu.load_assembly(&assembly::PROGRAM_SUM_OF_0_TO_100);

                cpu::run(&mut cpu, &mut frame_buffer, &mut terminal)?;

                frame_buffer.clear();

                /* Restore main menu */
                frame_buffer.push_msg(welcome.clone());

            }else if key.code == crossterm::event::KeyCode::Char('q') {
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

