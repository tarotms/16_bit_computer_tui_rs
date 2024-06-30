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
    register: [u16; 8],

    frequency: u64,
    cool_down: u64,

}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: ProgramCount::new(),
            ram: RAM::new(),
            register: [0; 8],
            frequency: 1,
            cool_down: 100,
        }
    }

    pub fn startup(&mut self) {
        let buffer_size = 512;
        let mut buffer_screen = String::with_capacity(buffer_size);
        let mut buffer_behind = String::with_capacity(buffer_size);
    


        let cd = std::time::Duration::from_millis(self.cool_down);

        loop {

            let start_time = std::time::Instant::now();
            let cycles = 16;

            /* kernel clock */
            for _ in 0..cycles {

                self.pc.increment();
            
                let pc_count = self.pc.print();

                buffer_behind.push_str(&utils::format(
                    "\x1B[2J\x1B[HProgram count Dec", format!("0b{:05}", pc_count))
                );

                buffer_behind.push_str(&utils::format(
                    "Program count Bin", format!("0b{:016b})", pc_count))
                );

                for i in 0..8 {
                    buffer_behind.push_str(&utils::format(
                    &format!("R{:01X}", i),format!("{:8b}", self.register[i])));

                }

                buffer_behind.push_str(&utils::format(
                    "Frequency", format!("{} Hz",  self.frequency())))

            }

            std::mem::swap(&mut buffer_screen, &mut buffer_behind);
            
            print!("{}", buffer_screen);

            buffer_behind.clear();

            std::thread::sleep(cd);
        
            let average_cycle_time  = start_time.elapsed().as_micros() as f64 / cycles as f64;
            self.frequency = (1_000_000.0 / average_cycle_time) as u64;
            
        }
    
    }

    //todo add more assembly
    fn assembly(&mut self, opcode: Opcode, arg1: u16, arg2: u16, dest: usize) {
        match opcode {
            Opcode::ADD => {
                let sum = self.register[arg1 as usize] + self.register[arg2 as usize];
                self.register[dest] = sum;
            },
            Opcode::SUB => {
                let difference = self.register[arg1 as usize] - self.register[arg2 as usize];
                self.register[dest] = difference;
            },
            Opcode::JMP => {
                self.pc.set(arg1);
            }
        }
    }

    pub fn frequency(&self) -> u64 {
        self.frequency
    }

}