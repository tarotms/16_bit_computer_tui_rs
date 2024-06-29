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
mod atomic;

use std::sync::atomic::Ordering;
use std::time::{Instant, Duration};
use program_count::ProgramCount;

fn main() {
    let mut timer = utils::Timer::new();

    utils::welcome();

    let mut pc = ProgramCount::new();

    let duration = Duration::from_secs(1);
    let start = Instant::now();

    while Instant::now() - start < duration {

        pc.update(&[false; 16], true, false, false);
        atomic::CLOCK_ITERATIONS.fetch_add(1, Ordering::SeqCst);
        
    }

    pc.echo();

    utils::msg(
        "NAND_CALL_COUNTS", 
        atomic::NAND_CALL_COUNTS.load(Ordering::SeqCst));

    utils::msg(
        "Number of clock iterations in one second:", 
        atomic::CLOCK_ITERATIONS.load(Ordering::SeqCst));

    timer.stop(&"Run time");
}