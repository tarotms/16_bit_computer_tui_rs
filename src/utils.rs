use core::panic;
use std::time::Instant;

pub struct Timer {
    inst: Instant,
    logs: String
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            inst: std::time::Instant::now(),
            logs: String::new()
        }
    }

    pub fn stop(&mut self, head: &str) {
        let duration = self.inst.elapsed();
        println!("{}", format(head, &format!("{:?}", duration)));
    }

    pub fn restart(&mut self, head: &str) {
        let duration = self.inst.elapsed();
        self.inst = std::time::Instant::now();
        let log: String = format(head, &format!("{:?}", duration));
        self.logs.push_str(&log);
    }
    
    pub fn print_logs(&self) {
        print!("{}", self.logs);
    }
}

pub fn format(head: &str, body: &str) -> String {
    format!("{:width$} -> {}\n", head, body, width = 20)
}

pub fn test_logic_gate(func: fn(bool, bool) -> bool) {
    println!("┌───┬───┬───┐");
    println!("│ A │ B │ O │");
    println!("├───┼───┼───┤");
    for num in 0..4 {
        let a = (num & 1) != 0;
        let b = (num & 2) != 0;
        let str_buffer = format!(
            "│ {} │ {} │ {} │",
            a as u8,
            b as u8,
            func(a, b) as u8);
        println!("{}", str_buffer);
    }
    println!("└───┴───┴───┘");
}
