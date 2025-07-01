use rusttype::{Font, Scale};

use crate::{
    style::{RawTransform, Transform},
    utils,
};

const FONT: &[u8] = include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf");

enum RenderMethod {
    Text(usize, usize, f32, String),
    Rectangle(usize, usize, usize, usize, u32),
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

    pub fn draw_text(
        &mut self,
        x: usize,
        y: usize,
        scale: f32,
        text: &str,
        // color: u32,
    ) {
        self.render_stack
            .push(RenderMethod::Text(x, y, scale, text.to_string()));
        let font = Font::try_from_bytes(FONT).unwrap();
        let (w, h) = utils::measure_text(&font, &text, Scale::uniform(scale));
        self.transform.width = self.transform.width.max(w as usize);
        self.transform.height = self.transform.height.max(h as usize);
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        self.render_stack
            .push(RenderMethod::Rectangle(x, y, width, height, color));
        self.transform.width = self.transform.width.max(width);
        self.transform.height = self.transform.height.max(height);
    }

    pub fn draw(&mut self) {
        for m in &self.render_stack {
            match m {
                &RenderMethod::Rectangle(pos_x, pos_y, width, height, color) => {
                    for x in self.transform.x + pos_x..self.transform.x + pos_x + width {
                        for y in self.transform.y + pos_y..self.transform.y + pos_y + height {
                            if self.buffer.len() > y * self.parent_height + x
                                && self.parent_width > x
                            {
                                self.buffer[y * self.parent_height + x] = color;
                            }
                        }
                    }
                }

                RenderMethod::Text(px, py, scale, text) => {
                    let font = Font::try_from_bytes(FONT).unwrap();
                    let scale = rusttype::Scale::uniform(*scale);
                    let v_metrics = font.v_metrics(scale);
                    let glyphs: Vec<_> = font
                        .layout(
                            text,
                            scale,
                            rusttype::point(
                                (self.transform.x + px) as f32,
                                (self.transform.y + py) as f32 + v_metrics.ascent,
                            ),
                        )
                        .collect();
                    for glyph in glyphs {
                        if let Some(bb) = glyph.pixel_bounding_box() {
                            glyph.draw(|x, y, v| {
                                let x = x as i32 + bb.min.x + *px as i32;
                                let y = y as i32 + bb.min.y + *py as i32;

                                if x >= 0
                                    && x < self.parent_width as i32
                                    && y >= 0
                                    && y < self.parent_height as i32
                                {
                                    let index =
                                        (y as usize * self.parent_width + x as usize) as usize;
                                    let intensity = (v * 255.0) as u32;
                                    let color = (intensity << 16) | (intensity << 8) | intensity;
                                    self.buffer[index] = color;
                                }
                            });
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
