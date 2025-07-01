pub struct RenderScope {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl RenderScope {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buffer: vec![0; w * h],
            width: w,
            height: h,
        }
    }

    pub fn draw_rect(&mut self) {
        for y in 0..200 {
            for x in 0..200 {
                self.buffer[y * self.height + x] = 0xFF00FF;
            }
        }
    }

    pub fn get_buffer(&self) -> Vec<u32> {
        self.buffer.clone()
    }
}
