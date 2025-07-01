use crate::style::{RawTransform, Transform};

enum RenderMethod {
    // Text(String),
    Rectangle(usize, usize, u32),
}

pub struct RenderScope {
    transform: RawTransform,
    render_stack: Vec<RenderMethod>,
    parent_width: usize,
    parent_height: usize,
    buffer: Vec<u32>,
}

impl RenderScope {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buffer: vec![0; w * h],
            render_stack: Vec::new(),
            parent_width: w,
            parent_height: h,
            transform: RawTransform::new(),
        }
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        transform.use_dimensions(&mut self.transform);
        transform.use_position(self.parent_width, self.parent_height, &mut self.transform);
    }

    // pub fn draw_text(&mut self, text: &str) {
    //     self.render_stack.push(RenderMethod::Text(text.to_string()));
    //     let (w, h) = utils::str_size(text);
    //     self.transform.width = self.transform.width.max(w);
    //     self.transform.height = self.transform.height.max(h);
    // }

    pub fn draw_rect(&mut self, width: usize, height: usize, color: u32) {
        self.render_stack
            .push(RenderMethod::Rectangle(width, height, color));
        self.transform.width = self.transform.width.max(width);
        self.transform.height = self.transform.height.max(height);
    }

    pub fn draw(&mut self) {
        for m in &self.render_stack {
            match m {
                &RenderMethod::Rectangle(width, height, color) => {
                    for y in 0..height {
                        for x in 0..width {
                            self.buffer[y * self.parent_height + x] = color;
                        }
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.render_stack.clear();
        self.transform.width = 0;
        self.transform.height = 0;
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.transform.width, self.transform.height)
    }

    pub fn get_parent_size(&self) -> (usize, usize) {
        (self.parent_width, self.parent_height)
    }

    pub fn get_buffer(&self) -> Vec<u32> {
        self.buffer.clone()
    }
}
