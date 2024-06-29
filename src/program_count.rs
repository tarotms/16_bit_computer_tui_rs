use crate::utils;
pub struct ProgramCount {
    count: u16,
}

impl ProgramCount {
    pub fn new() -> ProgramCount {

        ProgramCount {
            count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn set(&mut self, value: u16) {
        self.count = value;
    }

    /* replace manual implementation with built-in implementation */
    pub fn increment(&mut self) {
        let (new_count, overflowed) = self.count.overflowing_add(1);
        if overflowed {
            self.reset();
        } else {
            self.set(new_count);
        }
    }

    pub fn echo(&self) {
        utils::visualize16(self.count);
    }

}