
pub struct RAM {
    memory: [u16; 65536],
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            memory: [0; 64 * 1024],
        }
    }

    pub fn clear(&mut self) {
        self.memory = [0; 64 * 1024];
    }

    pub fn read(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }
}
