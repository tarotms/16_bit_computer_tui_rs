/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * www.youtube.com/@OnigirazuNori
 */

mod utils;
mod program_count;
mod ram;
mod cpu;


fn main() {

    utils::welcome();
    println!("> Press s to begin");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("s") {
    let mut cpu = cpu::CPU::new();
    cpu.startup();

    }

}

