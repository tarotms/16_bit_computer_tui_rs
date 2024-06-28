use crate::chip;
use crate::utils;
pub struct ProgramCount {
    count: [bool; 16],
    ones: [bool; 16],
}

impl ProgramCount {
    pub fn new() -> ProgramCount {
        let init: [bool; 16] = [false; 16];
        let mut ones_arr: [bool; 16] = [false; 16];
        ones_arr[15] = true;

        ProgramCount {
            count: init,
            ones: ones_arr
        }
    }

    pub fn update(&mut self, input: &[bool; 16], inc: bool, reset: bool, load: bool) {
        if reset {
            self.count.fill(false);
        } else if load {
            self.count = *input;
        } else if inc {
            self.increment();
        }
    }

    fn increment(&mut self) {
        
        let mut carry = true;
        let mut next_carry = false;
//为什么一次加了2
        for i in (0..16).rev() {
            let result = chip::full_adder(
                self.count[i], self.ones[i], carry);

            self.count[i] = result.0;
            next_carry = result.1;

            carry = next_carry;
        }

        println!("{:?}", self.count);

    }

    pub fn echo(&self) {
        utils::visualize16d(&self.count);
    }

}