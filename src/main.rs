/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 */

use tui::backend::Backend;
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
mod setting;

fn exec_program<B: Backend>(
    cpu: &mut cpu::CPU,
    program: &[(cpu::Opcode, u16, u16, u16)],
    frame_buffer: &mut FrameBuffer,
    terminal: &mut Terminal<B>,
    main_menu: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    cpu.reset();
    cpu.load_assembly(program);
    cpu::run(cpu, frame_buffer, terminal)?;

    /* Clear and restore main menu */
    frame_buffer.clear();
    frame_buffer.push_msg(main_menu.to_string());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    /* Initialization terminal */
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let main_menu = utils::main_menu();

    let mut frame_buffer = FrameBuffer::default();
    frame_buffer.push_msg(main_menu.clone());

    let mut cpu = cpu::CPU::new();
    
    /* todo 按键可以更新设置，设置需要click函数 */
    loop {
        frame_buffer.update_settings(cpu.settings.clone());

        terminal.draw(|f| {
                ui::ui(f, &frame_buffer)})?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('1') {
                exec_program(
                    &mut cpu,
                    &assembly::PROGRAM_SUM_OF_0_TO_100,
                    &mut frame_buffer,
                    &mut terminal,
                    &main_menu,
                )?;
            } else if key.code == KeyCode::Char('2') {
                exec_program(
                    &mut cpu,
                    &assembly::PROGRAM_FIBONACCI_10TH,
                    &mut frame_buffer,
                    &mut terminal,
                    &main_menu,
                )?;
            }else if key.code == KeyCode::Char('3') {
                exec_program(
                    &mut cpu,
                    &assembly::PROGRAM_FACTORIAL_5,
                    &mut frame_buffer,
                    &mut terminal,
                    &main_menu,
                )?;
            } else if key.code == KeyCode::Char('n') {
                cpu.settings.0.click();///////////////////////////////
                frame_buffer.update_settings(cpu.settings.clone());
            } else if key.code == crossterm::event::KeyCode::Char('q') {
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

