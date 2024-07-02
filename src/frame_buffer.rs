use crate::setting::Setting;

#[derive(Default)]
pub struct  FrameBuffer {
    pub buffer: Vec<String>,
    pub registers_led: [u16; 8],
    pub settings: Vec<Setting>,
}

impl FrameBuffer {
    pub fn push_msg(&mut self, msg: String) {
        self.buffer.extend(msg.lines().map(String::from));
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.registers_led = [0; 8];
    }

    pub fn update_registers(&mut self, registers: [u16; 8]) {
        self.registers_led = registers;
    }

    pub fn update_settings(&mut self, settings: Vec<Setting>) {
        self.settings = settings;
    }
}