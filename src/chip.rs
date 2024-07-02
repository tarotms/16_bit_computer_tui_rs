use crate::gate;

pub fn half_adder(
    a: bool,
    b: bool
) -> (bool, bool) {
    /* manual implementation */
    let sum = gate::xor(a, b);
    let carry = gate::and(a, b);
    (sum, carry)
}

pub fn full_adder(
    a: bool,
    b: bool,
    cin: bool
) -> (bool, bool) {
    
    /* manual implementation */
    let (sum_1, carry_1) = half_adder(a, b);
    let (sum_2, carry_2) = half_adder(sum_1, cin);
    let carry_out = gate::or(carry_1, carry_2);
    (sum_2, carry_out)
}


pub fn not16(x: u16) -> u16 {
    let mut result = 0u16;
    for i in 0..16 {
        let bit = (x >> i) & 1 == 1;
        let not_bit = gate::notx(bit, bit);
        if not_bit {
            result |= 1 << i;
        }
    }
    result
}

pub fn and16(x: u16, y: u16) -> u16 {
    let mut result = 0u16;
    for i in 0..16 {
        let x_bit = (x >> i) & 1 == 1;
        let y_bit = (y >> i) & 1 == 1;
        let and_bit = gate::and(x_bit, y_bit);
        if and_bit {
            result |= 1 << i;
        }
    }
    result
}

pub fn adder16(x: u16, y: u16, carry_in: bool) -> (u16, bool) {
    let mut result = 0u16;
    let mut carry = carry_in;

    for i in 0..16 {
        let x_bit = (x >> i) & 1 == 1;
        let y_bit = (y >> i) & 1 == 1;
        let (sum, carry_out) = full_adder(x_bit, y_bit, carry);
        if sum {
            result |= 1 << i;
        }
        carry = carry_out;
    }

    (result, carry)
}

pub fn alu(
    x: u16,
    y: u16,
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool
) -> (u16, bool, bool) {
    let mut x_processed = if zx { 0 } else { x };
    if nx {
        x_processed = not16(x_processed);
    }

    let mut y_processed = if zy { 0 } else { y };
    if ny {
        y_processed = not16(y_processed);
    }

    let mut result = if f {
        adder16(x_processed, y_processed, false).0
    } else {
        and16(x_processed, y_processed)
    };

    if no {
        result = not16(result);
    }

    let zr = result == 0;

    let ng = (result >> 15) & 1 == 1;

    (result, zr, ng)
}