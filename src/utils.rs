use std::sync::atomic::{AtomicUsize};

pub static NAND_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn welcome() -> String {
    let mut welcome_message = String::new();
    welcome_message += " _ _  _   _\n";
    welcome_message += r"(/\///_/// //";
    welcome_message += "\nRUST_NAND2TETRIS\n";
    welcome_message += "Created by Onigirazu Nori\n";
    welcome_message += "www.youtube.com/@OnigirazuNori\n\n";
    
    welcome_message
}

pub fn main_menu() -> String {
    let mut message: String = welcome();
    message += "Press 1 -> Calculate sum of numbers from 0 to 100\n";
    message += "Press 2 -> Calculate Fibonacci 10th\n";
    message += "Press 3 -> Calculate factorial of 7\n";
    message += "\nPress Q -> Quit\n";
    message
    
}

pub fn format<T: std::fmt::Display>(head: &str, body: T) -> String {
    format!("{:width$} -> {}\n", head, body, width = 20)
}



