use crate::program_count::ProgramCount;
use crate::ram::RAM;
use crate::utils;

#[derive(Clone, Copy, PartialEq)]
enum Opcode {
    LDI,
    LDR,
    STR,
    JMP,
    NOP,
    ADD,
    SUB,
    AND,
    OR,
    NOT,
    LE,
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
            cool_down: 10,

            commands: Vec::new(),
        };

        cpu.load_commands();

        cpu
    }

    pub fn startup(&mut self) {
        let buffer_size = 1024;
        let mut buffer_screen = String::with_capacity(buffer_size);
        let mut buffer_behind = String::with_capacity(buffer_size);
    
        let cd = std::time::Duration::from_millis(self.cool_down);

        'outer: loop {

            let start_time = std::time::Instant::now();
            let cycles = 16;

            /* kernel clock */
            for _ in 0..cycles {

                let pc_count = self.pc.get();

                let (opcode, dest, arg1, arg2) = self.commands.get(pc_count as usize).unwrap();

                if *opcode == Opcode::END {break 'outer;}
                self.exec(*opcode, *dest, *arg1, *arg2);

                buffer_behind.push_str("\x1B[2J\x1B[");

                buffer_behind.push_str(&utils::format(
                    "HProgram count Dec", format!("0b{:05}", pc_count))
                );

                buffer_behind.push_str(&utils::format(
                    "Program count Bin", format!("0b{:016b}", pc_count))
                );

                for i in 0..8 {
                    buffer_behind.push_str(&utils::format(
                    &format!("R 0x{:1X}", i),format!("0x{:04X} -> {:05}", self.register[i], self.register[i])));

                }

                buffer_behind.push_str(&utils::format(
                    "Frequency", format!("{} Hz",  self.frequency())));


                std::mem::swap(&mut buffer_screen, &mut buffer_behind);

                print!("{}", buffer_screen);

                buffer_behind.clear();
    
                std::thread::sleep(cd);

                self.pc.increment();

            }

            let average_cycle_time  = start_time.elapsed().as_micros() as f64 / cycles as f64;
            self.frequency = (1_000_000.0 / average_cycle_time) as u64;
            
        }
    
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
            Opcode::OR  => self.register[dest as usize] = self.register[arg1 as usize] | self.register[arg2 as usize],
            Opcode::NOT => self.register[dest as usize] = !self.register[arg1 as usize],
            Opcode::LE  => self.register[dest as usize] = (self.register[arg1 as usize] < self.register[arg2 as usize]) as u16,
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
        self.load_command((Opcode::LE , 4, 1, 3));//寄存器4 = 寄存器2 < 寄存器3
        self.load_command((Opcode::ADD, 1, 1, 2));//寄存器1 += 寄存器2
        self.load_command((Opcode::JNE, 4, 3, 0));//如果寄存器4不等于0 则跳转至第4行命令

        self.load_command((Opcode::END, 0, 0, 0));

    }

}