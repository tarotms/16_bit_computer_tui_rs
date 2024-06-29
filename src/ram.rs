
pub struct Ram {
    memory: [u16; 65536],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0; 65536],
        }
    }

    pub fn read(&self, address: usize) -> u16 {
        self.memory[address]
    }

    pub fn write(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }
}