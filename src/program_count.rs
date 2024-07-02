use crate::chip;

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

    /* Built-in implementation */
    pub fn increment(&mut self) {
        let (new_count, overflowed) = self.count.overflowing_add(1);
        if overflowed {
            self.reset();
        } else {
            self.set(new_count);
        }
    }

    /* Manual implementation */
    pub fn increment_nand(&mut self) {
        let mut carry_in = true;
        let mut new_count = 0u16;

        for i in 0..16 {
            let bit = (self.count >> i) & 1 == 1;
            let (sum, carry_out) = chip::full_adder(bit, false, carry_in);
            if sum {
                new_count |= 1 << i;
            }
            carry_in = carry_out;
        }

        if carry_in {
            self.reset();
        } else {
            self.set(new_count);
        }
    }

    pub fn get(&self) -> u16 {
        self.count
    }

}