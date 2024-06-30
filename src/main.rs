/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 */

mod utils;
mod program_count;
mod ram;
mod cpu;
mod gate;

fn main() {

    utils::welcome();
    utils::msg("System", "Press s to begin");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("s") {
        let mut cpu = cpu::CPU::new();
        cpu.startup();
    }

    utils::msg("System", "exit");

}

