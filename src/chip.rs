//use crate::gate;

pub fn _half_adder(
    a: bool,
    b: bool
) -> (bool, bool) {
    /* manual implementation */
    //let sum = gate::xor(a, b);
    //let carry = gate::and(a, b);
    //(sum, carry)

    /* built-in implementatiln */
    (a^b, a&&b)
}

pub fn _full_adder(
    a: bool,
    b: bool,
    cin: bool
) -> (bool, bool) {
    
    /* manual implementation */
    //let (sum_1, carry_1) = half_adder(a, b);
    //let (sum_2, carry_2) = half_adder(sum_1, cin);
    //let carry_out = gate::or(carry_1, carry_2);
    //(sum_2, carry_out)

    /* built-in implementatiln */
    let (sum_1, _) = _half_adder(a, b);
    let (sum_2, _) = _half_adder(sum_1, cin);
    (sum_2, a || b)
}

