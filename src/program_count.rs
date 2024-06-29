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

    pub fn update(&mut self, input: u16, inc: bool, reset: bool, load: bool) {
        if reset {
            self.count = 0;
        } else if load {
            self.count = input;
        } else if inc {
            self.increment();
        }
    }

    /* replace manual implementation with built-in implementation */
    fn increment(&mut self) {
        let (new_count, overflowed) = self.count.overflowing_add(1);
        if overflowed {
            self.count = 0;
        } else {
            self.count = new_count;
        }
    }

    pub fn echo(&self) {
        utils::visualize16(self.count);
    }

}