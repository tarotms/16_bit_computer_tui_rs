use crate::program_count::ProgramCount;
use crate::ram::RAM;
use crate::utils;

enum Opcode {
    ADD,
    SUB,
    JMP,
}
pub struct CPU {

    pc: ProgramCount,
    /* 64KB memory */
    ram: RAM,

    frequency: u64,
    cool_down: u64,

}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: ProgramCount::new(),
            ram: RAM::new(),
            frequency: 1,
            cool_down: 300,
        }
    }

    pub fn startup(&mut self) {
        let cd = std::time::Duration::from_micros(self.cool_down);

        loop {

            let start_time = std::time::Instant::now();
            let cycles = 512;

            /* kernel clock */
            for _ in 0..cycles {
                self.pc.increment();
            
                std::thread::sleep(cd);

                print!("\x1B[2J\x1B[H");
                self.pc.echo();
                self.show_frequency();

            }

            let average_cycle_time  = start_time.elapsed().as_micros() as f64 / cycles as f64;
            self.frequency = (1_000_000.0 / average_cycle_time) as u64;
            
        }
    
    }

    pub fn show_frequency(&self) {
        utils::msg("Calculated Frequency", format!("{} Hz", self.frequency));
    }

}