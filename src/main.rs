/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * www.youtube.com/@OnigirazuNori
 */

mod utils;
mod program_count;
mod atomic;
mod ram;
mod cpu;

use std::sync::atomic::Ordering;

fn main() {

    utils::welcome();
    println!("> Press s to begin");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("s") {
    let mut cpu = cpu::CPU::new();
    cpu.startup();

    utils::msg(
        "Number of clock iterations in one second:", 
        atomic::CLOCK_ITERATIONS.load(Ordering::SeqCst));
    }

}

