#[derive(Debug, Clone, Copy)]

pub enum Setting {
    NandMode(bool),
    
}

impl Setting {
    pub fn description(&self) -> &str {
        match *self {
            Setting::NandMode(_) => "Nand mode"
        }
    }

    pub fn value(&self) -> String {
        match *self {
            Setting::NandMode(val) => if val { "NAND".to_string() } else { "Built-in".to_string() },
        }
    }

    pub fn toggle(&mut self) {
        match *self {
            Setting::NandMode(ref mut val) => *val = !*val,
        }
    }
}