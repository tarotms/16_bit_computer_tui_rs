use crate::program_count::ProgramCount;
use crate::ram::RAM;
use crate::utils;
use crate::ui;
use crate::frame_buffer::FrameBuffer;
use crate::setting::Setting;

use tui::{
    backend::Backend,Terminal,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Opcode {
    LDI,
    LDR,
    STR,
    JMP,
    NOP,
    ADD,
    SUB,
    MUL,
    AND,
    BOR,
    NOT,
    LTE,
    JEQ,
    JNE,
    MOV,
    END,
}
pub struct CPU {

    pc: ProgramCount,
    /* 64KB memory */
    ram: RAM,
    register: [u16; 8],

    frequency: u64,

    commands: Vec<(Opcode, u16, u16, u16)>,

    pub settings: Vec<Setting>

}

impl CPU {
    pub fn new() -> CPU {
        let cpu = CPU {
            pc: ProgramCount::new(),
            ram: RAM::new(),
            register: [0; 8],
            
            frequency: 1,

            commands: Vec::new(),

            settings: vec![
                Setting::new("Nand Mode", false, "NAND", "Built-in"),
                ],
        };

        cpu
    }
    
    pub fn reset(&mut self) {
        self.pc.reset();
        self.ram.clear();
        self.register = [0; 8];
        self.commands.clear();
    }

    pub fn frequency(&self) -> u64 {
        self.frequency
    }

    /* only nand gate implementation */

    fn exec(&mut self, opcode: Opcode, dest: u16, arg1: u16, arg2: u16) -> bool {
        match opcode {
            Opcode::LDI => self.register[dest as usize] = arg1,
            Opcode::LDR => self.register[dest as usize] = self.ram.read(arg1),
            Opcode::STR => self.ram.write(arg1, self.register[dest as usize]),
            Opcode::JMP => { self.pc.set(arg1); return false; },
            Opcode::NOP => {},
            Opcode::ADD => self.register[dest as usize] = self.register[arg1 as usize] + self.register[arg2 as usize],
            Opcode::SUB => self.register[dest as usize] = self.register[arg1 as usize] - self.register[arg2 as usize],
            Opcode::MUL => self.register[dest as usize] = self.register[arg1 as usize] * self.register[arg2 as usize],
            Opcode::AND => self.register[dest as usize] = self.register[arg1 as usize] & self.register[arg2 as usize],
            Opcode::BOR => self.register[dest as usize] = self.register[arg1 as usize] | self.register[arg2 as usize],
            Opcode::NOT => self.register[dest as usize] = !self.register[arg1 as usize],
            Opcode::LTE => self.register[dest as usize] = (self.register[arg1 as usize] < self.register[arg2 as usize]) as u16,
            Opcode::JEQ => if self.register[dest as usize] == 0 { self.pc.set(arg1); return false;},
            Opcode::JNE => if self.register[dest as usize] != 0 { self.pc.set(arg1); return false;},
            Opcode::MOV => self.register[dest as usize] = self.register[arg1 as usize],
            Opcode::END => {},
        }
        true
    }

    fn load_command(&mut self, command: (Opcode, u16, u16, u16)) {
        self.commands.push(command);
    }
    
    pub fn load_assembly(&mut self, commands: &[(Opcode, u16, u16, u16)]) {
        for command in commands.iter() {
            self.load_command(*command);
        }
    }

    pub fn setting_toggle(&mut self, index: usize) {
        self.settings[index].toggle();
    }

}

pub fn run<B: Backend>(
    cpu: &mut CPU, 
    frame_buffer: &mut FrameBuffer,
    terminal: &mut Terminal<B>
) -> Result<(), Box<dyn std::error::Error>> {
    let buffer_size = 1024;
    let mut buffer_screen = String::with_capacity(buffer_size);

    let welcome = utils::welcome();

    'outer: loop {

        let start_time = std::time::Instant::now();
        let cycles = 16;

        /* kernel clock */
        for _ in 0..cycles {

            let pc_count = cpu.pc.get();

            frame_buffer.update_nand_called_count();

            let (opcode, dest, arg1, arg2) = cpu.commands.get(pc_count as usize).cloned().unwrap();

            if opcode == Opcode::END {break 'outer;}

            let should_increment = cpu.exec(opcode, dest, arg1, arg2);

            buffer_screen.push_str(&welcome);

            buffer_screen.push_str(&utils::format(
                "Program count Dec", format!("0b{:05}", pc_count))
            );

            buffer_screen.push_str(&utils::format(
                "Program count Bin", format!("0b{:016b}", pc_count))
            );

            let command_fotmat = format!(
                "\nCurrent command: {:?} {} {:05} {:05}\n\n",
                opcode, dest, arg1, arg2
            );

            buffer_screen.push_str(&command_fotmat);

            for i in 0..8 {
                buffer_screen.push_str(&utils::format(
                &format!("R 0x{:1X}", i),format!("0x{:04X} -> {:05}", cpu.register[i], cpu.register[i])));

            }
            
            buffer_screen.push_str(&utils::format(
                "Frequency", format!("{} Hz",  cpu.frequency())));

            frame_buffer.clear();
            frame_buffer.push_msg(buffer_screen.clone());
            frame_buffer.update_registers(cpu.register);
            frame_buffer.update_settings(cpu.settings.clone());

            terminal.draw(|f| ui::ui(f, frame_buffer))?;

            buffer_screen.clear();

            if should_increment {
                if cpu.settings[0].get() {
                    cpu.pc.increment_nand();
                } else {
                    cpu.pc.increment();
                }
            }
        }

        let average_cycle_time  = start_time.elapsed().as_micros() as f64 / cycles as f64;
        cpu.frequency = (1_000_000.0 / average_cycle_time) as u64;
        
    }

    frame_buffer.push_msg(String::from("\nProgram returned 'Ok(())'"));
    frame_buffer.push_msg(String::from("Press Q -> Back to main menu"));

    terminal.draw(|f| ui::ui(f, frame_buffer))?;

    loop {
        terminal.draw(|f| ui::ui(f, &frame_buffer))?;
    
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.code == crossterm::event::KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}

