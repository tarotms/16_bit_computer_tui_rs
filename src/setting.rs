#[derive(Debug, Clone, Copy)]
pub struct Setting {
    description: &'static str,
    switch: bool,
    name_on: &'static str,
    name_off: &'static str,
}

impl Setting {
    pub fn new(
        description: &'static str,
        switch: bool,
        name_on: &'static str,
        name_off: &'static str
    ) -> Self {
        Setting {
            description,
            switch,
            name_on,
            name_off,
        }
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn value(&self) -> &str {
        if self.switch {
            self.name_on
        } else {
            self.name_off
        }
    }

    pub fn toggle(&mut self) {
        self.switch = !self.switch;
    }

    pub fn get(&self) -> bool {
        self.switch
    }
}