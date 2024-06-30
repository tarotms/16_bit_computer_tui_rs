use crate::program_count::ProgramCount;
use crate::ram::RAM;
use crate::utils;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Opcode {
    LDI,
    LDR,
    STR,
    JMP,
    NOP,
    ADD,
    SUB,
    AND,
    BOR,
    NOT,
    LTE,
    JEQ,
    JNE,
    END,
}
pub struct CPU {

    pc: ProgramCount,
    /* 64KB memory */
    ram: RAM,
    register: [u16; 8],

    frequency: u64,
    cool_down: u64,

    commands: Vec<(Opcode, u16, u16, u16)>,

}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            pc: ProgramCount::new(),
            ram: RAM::new(),
            register: [0; 8],
            
            frequency: 1,
            cool_down: 0,

            commands: Vec::new(),
        };

        cpu.load_commands();

        cpu
    }

    pub fn frequency(&self) -> u64 {
        self.frequency
    }

    fn exec(&mut self, opcode: Opcode, dest: u16, arg1: u16, arg2: u16) {
        match opcode {
            Opcode::LDI => self.register[dest as usize] = arg1,
            Opcode::LDR => self.register[dest as usize] = self.ram.read(arg1),
            Opcode::STR => self.ram.write(arg1, self.register[dest as usize]),
            Opcode::JMP => self.pc.set(arg1),
            Opcode::NOP => {},
            Opcode::ADD => self.register[dest as usize] = self.register[arg1 as usize] + self.register[arg2 as usize],
            Opcode::SUB => self.register[dest as usize] = self.register[arg1 as usize] - self.register[arg2 as usize],
            Opcode::AND => self.register[dest as usize] = self.register[arg1 as usize] & self.register[arg2 as usize],
            Opcode::BOR  => self.register[dest as usize] = self.register[arg1 as usize] | self.register[arg2 as usize],
            Opcode::NOT => self.register[dest as usize] = !self.register[arg1 as usize],
            Opcode::LTE => self.register[dest as usize] = (self.register[arg1 as usize] < self.register[arg2 as usize]) as u16,
            Opcode::JEQ => if self.register[dest as usize] == 0 { self.pc.set(arg1); },
            Opcode::JNE => if self.register[dest as usize] != 0 { self.pc.set(arg1); },
            Opcode::END => {},
        }
    }

    fn load_command(&mut self, command: (Opcode, u16, u16, u16)) {
        self.commands.push(command);
    }

    fn load_commands(&mut self) {
        self.load_command((Opcode::LDI, 0, 0, 0));//寄存器0 = 0 保存结果
        self.load_command((Opcode::LDI, 1, 1, 0));//寄存器1 = 1 当前循环次数
        self.load_command((Opcode::LDI, 2, 1, 0));//寄存器2 = 1 单位1
        self.load_command((Opcode::LDI, 3, 100, 0));//寄存器3 = 101

        self.load_command((Opcode::ADD, 0, 0, 1));//寄存器0 += 寄存器1
        self.load_command((Opcode::LTE, 4, 1, 3));//寄存器4 = 寄存器2 < 寄存器3
        self.load_command((Opcode::ADD, 1, 1, 2));//寄存器1 += 寄存器2
        self.load_command((Opcode::JNE, 4, 3, 0));//如果寄存器4不等于0 则跳转至第4行命令

        self.load_command((Opcode::END, 0, 0, 0));

    }

}

pub fn startup<B: Backend>(cpu: &mut CPU, terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let buffer_size = 1024;
    let mut buffer_screen = String::with_capacity(buffer_size);
    let mut buffer_behind = String::with_capacity(buffer_size);

    let cd = std::time::Duration::from_millis(cpu.cool_down);

    let welcome = utils::welcome();

    'outer: loop {

        let start_time = std::time::Instant::now();
        let cycles = 16;

        /* kernel clock */
        for _ in 0..cycles {

            let pc_count = cpu.pc.get();

            let (opcode, dest, arg1, arg2) = cpu.commands.get(pc_count as usize).cloned().unwrap();

            if opcode == Opcode::END {break 'outer;}

            cpu.exec(opcode, dest, arg1, arg2);

            buffer_behind.push_str(&welcome);

            buffer_behind.push_str(&utils::format(
                "Program count Dec", format!("0b{:05}", pc_count))
            );

            buffer_behind.push_str(&utils::format(
                "Program count Bin", format!("0b{:016b}", pc_count))
            );

            let command_fotmat = format!(
                "\nCurrent command: {:?} {} {:05} {:05}\n\n",
                opcode, dest, arg1, arg2
            );

            buffer_behind.push_str(&command_fotmat);

            for i in 0..8 {
                buffer_behind.push_str(&utils::format(
                &format!("R 0x{:1X}", i),format!("0x{:04X} -> {:05}", cpu.register[i], cpu.register[i])));

            }

            buffer_behind.push_str(&utils::format(
                "Frequency", format!("{} Hz",  cpu.frequency())));


            std::mem::swap(&mut buffer_screen, &mut buffer_behind);

            terminal.draw(|f| {
                ui(f, &buffer_screen);
            })?;

            buffer_behind.clear();

            std::thread::sleep(cd);

            cpu.pc.increment();

        }

        let average_cycle_time  = start_time.elapsed().as_micros() as f64 / cycles as f64;
        cpu.frequency = (1_000_000.0 / average_cycle_time) as u64;
        
    }

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let mut text: Vec<Spans> = buffer_screen
                .lines()
                .map(|line| Spans::from(Span::raw(line)))
                .collect();

            text.push(Spans::from(Span::raw("")));
            text.push(Spans::from(Span::raw("Program returned 'Ok(())'")));
            text.push(Spans::from(Span::raw("Press 'q' to exit.")));
            
            let paragraph = Paragraph::new(text)
                .block(Block::default().title(" CPU ").borders(Borders::ALL));
            f.render_widget(paragraph, chunks[0]);
        })?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.code == crossterm::event::KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, content: &str) {
    let size = f.size();

    let block = Block::default()
        .title("TUI Example")
        .borders(Borders::ALL);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let text: Vec<Spans> = content.lines().map(|line| Spans::from(Span::raw(line))).collect();
    let paragraph = Paragraph::new(text)
        .block(Block::default().title(" CPU ").borders(Borders::ALL));
    f.render_widget(paragraph, chunks[0]);
}