pub struct Framebuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.buffer.iter_mut().for_each(|pixel| *pixel = color);
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = 0xFFFFFF;
        }
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}
