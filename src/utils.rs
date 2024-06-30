pub fn welcome() -> String {
    let mut welcome_message = String::new();
    welcome_message += " _ _  _   _\n";
    welcome_message += r"(/\///_/// //";
    welcome_message += "\nRUST_NAND2TETRIS\n";
    welcome_message += "Created by Onigirazu Nori\n";
    welcome_message += "www.youtube.com/@OnigirazuNori\n\n";
    
    welcome_message
}

pub fn format<T: std::fmt::Display>(head: &str, body: T) -> String {
    format!("{:width$} -> {}\n", head, body, width = 20)
}



