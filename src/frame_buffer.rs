const ON: u32 = 0x9a8c98;
const OFF: u32 = 0x22223b;

pub struct FrameBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![OFF; width * height],
            width,
            height,
        }
    }

    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        let l = y * self.width + x;
        assert!(l < self.width * self.height);

        if self.buffer[l] == ON {
            self.buffer[l] = OFF;
        } else {
            self.buffer[l] = ON;
        }
    }

    pub fn buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}
