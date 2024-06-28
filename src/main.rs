/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * www.youtube.com/@OnigirazuNori
 */

mod gate;
mod utils;
mod chip;

fn main() {
    let mut timer = utils::Timer::new();

    /* todo : fix adder16 */

    let a = utils::u16_to_bool_array(0b0000000000000001);
    let b = utils::u16_to_bool_array(0b0000010000000001);
    let initial_carry = false;
    let (result, carry) = chip::adder16(&a, &b, initial_carry);
    utils::visualize16(&result);

    timer.stop(&"Run time");
}