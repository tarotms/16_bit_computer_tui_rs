pub struct Register {
    value: u16
}

impl Register {
    pub fn new() -> Register {
        Register {
            value: 0
        }
    }

    pub fn load(&mut self, data: u16) {
        self.value = data;
    }

    pub fn read(&self) -> u16 {
        self.value
    }
}