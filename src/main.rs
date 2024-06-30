/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 */

 use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Constraint, Direction, Layout},
    Terminal,
};


mod utils;
mod program_count;
mod ram;
mod cpu;
mod gate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut welcome = utils::welcome();
    welcome += "Press 's' to startup\n";

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);
            let welcome_paragraph = Paragraph::new(welcome.as_ref())
                .block(Block::default().title(" CPU ").borders(Borders::ALL));

            f.render_widget(welcome_paragraph, chunks[0]);

        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('s') {
                let mut cpu = cpu::CPU::new();
                cpu::startup(&mut cpu, &mut terminal)?;
                break;
            }
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    
    terminal.show_cursor()?;

    Ok(())
}

