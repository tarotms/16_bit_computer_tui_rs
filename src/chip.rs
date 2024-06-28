use crate::{gate, utils::visualize16};

pub fn half_adder(
    a: bool,
    b: bool
) -> (bool, bool) {
    let sum = gate::xor(a, b);
    let carry = gate::and(a, b);
    (sum, carry)
}

pub fn full_adder(
    a: bool,
    b: bool,
    cin: bool
) -> (bool, bool) {
    let (sum_1, carry_1) = half_adder(a, b);
    let (sum_2, carry_2) = half_adder(sum_1, cin);
    let carry_out = gate::or(carry_1, carry_2);
    (sum_2, carry_out)
}

pub fn adder16(
    a: &[bool; 16],
    b: &[bool; 16],
    initial_carry: bool
) -> ([bool; 16], bool) {

    let mut sum = [false; 16];
    let mut carry = initial_carry;
    let mut next_carry;

    for i in (0..16).rev() {
        let result = full_adder(a[i], b[i], carry);
        sum[i] = result.0;
        next_carry = result.1;
        carry = next_carry;
    }

    (sum, carry)
}

pub fn increment16(a: &[bool; 16]) -> [bool; 16] {
    let b = [true, false, false, false, false, false, false, false,
             false, false, false, false, false, false, false, false];

    let (sum, _carry_out) = adder16(a, &b, false);
    sum
}

pub fn alu16(
    x: &[bool; 16],
    y: &[bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool
) -> ([bool; 16], bool, bool) {
    let mut x_processed = if zx { [false; 16] } else { *x };
    if nx {
        x_processed = gate::not16(&x_processed);
    }

    let mut y_processed = if zy { [false; 16] } else { *y };
    if ny {
        y_processed = gate::not16(&y_processed);
    }

    let initial_carry = gate::and(ny, f);

    let mut result = if f {
        adder16(&x_processed, &y_processed, initial_carry).0
    } else {
        gate::and16(&x_processed, &y_processed)
    };

    if no {
        result = gate::not16(&result);
    }

    let zr = result.iter().all(|&bit| !bit);
    let ng = result[0];

    (result, zr, ng)
}

