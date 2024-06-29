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


use std::sync::atomic::Ordering;
use program_count::ProgramCount;

fn main() {
    let mut timer = utils::Timer::new();
    /**/
    let mut pc = ProgramCount::new();

    pc.echo();

    for _ in 0..16 {

        pc.update(&[false; 16], true, false, false);

        pc.echo();

    }

    println!("NAND was called {} times.", gate::NAND_CALL_COUNT.load(Ordering::SeqCst));

    timer.stop(&"Run time");
}