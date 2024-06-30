
pub fn separator() {
    println!("");
}

pub fn welcome() {
    println!(" _ _  _   _");
    println!(r"(/\///_/// //");
    println!("RUST_NAND2TETRIS");
    println!("Created by Onigirazu Nori");
    println!("www.youtube.com/@OnigirazuNori");
    separator();
}

pub fn format<T: std::fmt::Display>(head: &str, body: T) -> String {
    format!("{:width$} -> {}\n", head, body, width = 20)
}

pub fn msg<T: std::fmt::Display>(head: &str, body: T) {
    let text: String = format(head, body);
    print!("{}", text);
}


