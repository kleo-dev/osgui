use rusttype::{Font, Scale};

use crate::{
    style::{RawTransform, Transform},
    utils,
};

const FONT: &[u8] = include_bytes!("../assets/DejaVuSans.ttf");

#[derive(Debug, Clone)]
pub enum RenderMethod {
    Text(String, usize, usize, f32, u32),
    Rectangle(usize, usize, usize, usize, u32),
}

pub struct RenderScope {
    transform: RawTransform,
    render_stack: Vec<RenderMethod>,
    parent_width: usize,
    parent_height: usize,
    buffer: Vec<Vec<u32>>,
}

impl RenderScope {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buffer: vec![vec![0; w]; h],
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

    pub fn draw_text(&mut self, x: usize, y: usize, scale: f32, text: &str, color: u32) {
        self.render_stack
            .push(RenderMethod::Text(text.to_string(), x, y, scale, color));
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
        let mut tmp = self.buffer.clone();
        self.draw_buf(&mut tmp, 0, 0);
        self.buffer = tmp;
    }

    pub fn draw_buf(&self, buf: &mut Vec<Vec<u32>>, offset_x: usize, offset_y: usize) {
        let offset_x = self.transform.x + offset_x;
        let offset_y = self.transform.y + offset_y;

        for m in &self.render_stack {
            match m {
                &RenderMethod::Rectangle(px, py, width, height, color) => {
                    for y in offset_y + py..(offset_y + py + height).min(self.parent_height) {
                        for x in offset_x + px..(offset_x + px + width).min(self.parent_width) {
                            buf[y][x] = color;
                        }
                    }
                }

                RenderMethod::Text(text, px, py, scale, base_color) => {
                    let font = Font::try_from_bytes(FONT).unwrap();
                    let scale = Scale::uniform(*scale);
                    let v_metrics = font.v_metrics(scale);

                    let glyphs: Vec<_> = font
                        .layout(
                            text,
                            scale,
                            rusttype::point(
                                (offset_x + px) as f32,
                                (offset_y + py) as f32 + v_metrics.ascent,
                            ),
                        )
                        .collect();

                    for glyph in glyphs {
                        if let Some(bb) = glyph.pixel_bounding_box() {
                            glyph.draw(|gx, gy, v| {
                                let x = gx as i32 + bb.min.x + *px as i32;
                                let y = gy as i32 + bb.min.y + *py as i32;

                                if (0..self.parent_width as i32).contains(&x)
                                    && (0..self.parent_height as i32).contains(&y)
                                {
                                    let (x, y) = (x as usize, y as usize);

                                    let r_base = ((base_color >> 16) & 0xFF) as f32;
                                    let g_base = ((base_color >> 8) & 0xFF) as f32;
                                    let b_base = (base_color & 0xFF) as f32;

                                    let r = (r_base * v) as u32;
                                    let g = (g_base * v) as u32;
                                    let b = (b_base * v) as u32;

                                    buf[y][x] = (r << 16) | (g << 8) | b;
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

    pub fn get_size_or_parent(&self) -> (usize, usize) {
        (
            if self.transform.width > 0 {
                self.transform.width
            } else {
                self.parent_width
            },
            if self.transform.height > 0 {
                self.transform.height
            } else {
                self.parent_height
            },
        )
    }

    pub fn get_parent_size(&self) -> (usize, usize) {
        (self.parent_width, self.parent_height)
    }

    pub fn get_buffer1d(&self) -> Vec<u32> {
        self.buffer
            .iter()
            .flat_map(|row| row.iter().copied())
            .collect()
    }

    pub fn get_buffer(&self) -> Vec<Vec<u32>> {
        self.buffer.clone()
    }

    pub fn get_buffer_mut(&mut self) -> &mut Vec<Vec<u32>> {
        &mut self.buffer
    }
}
