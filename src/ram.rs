
pub struct RAM {
    memory: [u16; 65536],
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
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
