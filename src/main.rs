/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * www.youtube.com/@OnigirazuNori
 */

mod gate;
mod utils;
mod chip;
mod program_count;

use program_count::ProgramCount;

fn main() {
    let mut timer = utils::Timer::new();

    let mut pc = ProgramCount::new();

    pc.echo();

    for _ in 0..10 {

        pc.update(&[false; 16], true, false, false);

        pc.echo();

    }

    timer.stop(&"Run time");
}