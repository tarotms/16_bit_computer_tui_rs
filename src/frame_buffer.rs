#[derive(Default)]
pub struct  FrameBuffer {
    pub buffer: Vec<String>,
}

impl FrameBuffer {
    pub fn push_msg(&mut self, msg: String) {
        self.buffer.extend(msg.lines().map(String::from));
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}