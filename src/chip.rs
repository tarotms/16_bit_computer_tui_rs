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
    let (sum_1, carry_1) = half_adder(a, b);
    let (sum_2, carry_2) = half_adder(sum_1, cin);
    let carry_out = gate::or(carry_1, carry_2);
    (sum_2, carry_out)
}
