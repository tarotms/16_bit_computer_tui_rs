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


    utils::test_logic_gate(gate::nor);


    timer.stop(&"Run time");
}
