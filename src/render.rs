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
    RoundedRectangle(usize, usize, usize, usize, usize, u32),
    Merge(RenderScope),
}

#[derive(Debug, Clone)]
pub struct RenderScope {
    transform: RawTransform,
    render_stack: Vec<RenderMethod>,
    parent_width: usize,
    parent_height: usize,
    max_size: (usize, usize),
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
            max_size: (0, 0),
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, scale: f32, text: &str, color: u32) {
        if color == 0 {
            return;
        }
        self.render_stack
            .push(RenderMethod::Text(text.to_string(), x, y, scale, color));
        let font = Font::try_from_bytes(FONT).unwrap();
        let (w, h) = utils::measure_text(&font, &text, Scale::uniform(scale));
        self.transform.width = self.transform.width.max(w as usize);
        self.transform.height = self.transform.height.max(h as usize);
        self.update_size();
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        if color == 0 {
            return;
        }
        self.render_stack
            .push(RenderMethod::Rectangle(x, y, width, height, color));
        self.transform.width = self.transform.width.max(width);
        self.transform.height = self.transform.height.max(height);
        self.update_size();
    }

    pub fn draw_rect_rounded(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        radius: usize,
        color: u32,
    ) {
        if color == 0 {
            return;
        }
        if radius == 0 {
            self.draw_rect(x, y, width, height, color);
            return;
        }
        self.render_stack.push(RenderMethod::RoundedRectangle(
            x, y, width, height, radius, color,
        ));
        self.transform.width = self.transform.width.max(width);
        self.transform.height = self.transform.height.max(height);
        self.update_size();
    }

    pub fn merge(&mut self, scope: RenderScope) {
        let (w, h) = scope.get_max_size();
        self.render_stack.push(RenderMethod::Merge(scope));
        self.transform.width = self.transform.width.max(w as usize);
        self.transform.height = self.transform.height.max(h as usize);
        self.update_size();
    }
}

// After draw
impl RenderScope {
    pub fn draw(&mut self) {
        let mut tmp = std::mem::take(&mut self.buffer);
        self.draw_buf(&mut tmp);
        self.buffer = tmp;
    }

    pub fn draw_buf(&self, buf: &mut Vec<Vec<u32>>) {
        let offset_x = self.transform.x;
        let offset_y = self.transform.y;

        for m in &self.render_stack {
            match m {
                &RenderMethod::Rectangle(px, py, width, height, color) => {
                    for y in offset_y + py..(offset_y + py + height).min(self.parent_height) {
                        for x in offset_x + px..(offset_x + px + width).min(self.parent_width) {
                            buf[y][x] = color;
                        }
                    }
                }

                &RenderMethod::RoundedRectangle(px, py, width, height, radius, color) => {
                    let radius = radius as i32;
                    let w = width as i32;
                    let h = height as i32;
                    let start_x = offset_x as i32 + px as i32;
                    let start_y = offset_y as i32 + py as i32;

                    for y in 0..h {
                        for x in 0..w {
                            let dx = x;
                            let dy = y;

                            let in_top_left = dx < radius && dy < radius;
                            let in_top_right = dx >= w - radius && dy < radius;
                            let in_bottom_left = dx < radius && dy >= h - radius;
                            let in_bottom_right = dx >= w - radius && dy >= h - radius;

                            let dist = |cx: i32, cy: i32| -> bool {
                                let dx = dx - cx;
                                let dy = dy - cy;
                                dx * dx + dy * dy <= radius * radius
                            };

                            let draw = match (
                                in_top_left,
                                in_top_right,
                                in_bottom_left,
                                in_bottom_right,
                            ) {
                                (true, _, _, _) => dist(radius - 1, radius - 1),
                                (_, true, _, _) => dist(w - radius, radius - 1),
                                (_, _, true, _) => dist(radius - 1, h - radius),
                                (_, _, _, true) => dist(w - radius, h - radius),
                                _ => true, // middle or edges outside corner
                            };

                            if draw {
                                let global_x = start_x + x;
                                let global_y = start_y + y;
                                if global_x >= 0
                                    && global_x < self.parent_width as i32
                                    && global_y >= 0
                                    && global_y < self.parent_height as i32
                                {
                                    buf[global_y as usize][global_x as usize] = color;
                                }
                            }
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

                    let r_base = ((base_color >> 16) & 0xFF) as f32;
                    let g_base = ((base_color >> 8) & 0xFF) as f32;
                    let b_base = (base_color & 0xFF) as f32;

                    for glyph in glyphs {
                        if let Some(bb) = glyph.pixel_bounding_box() {
                            glyph.draw(|gx, gy, v| {
                                if v < 0.5 {
                                    return;
                                }

                                let x = (gx as i32 + bb.min.x + *px as i32) as usize;
                                let y = (gy as i32 + bb.min.y + *py as i32) as usize;

                                if let Some(row) = buf.get_mut(y) {
                                    if let Some(col) = row.get_mut(x) {
                                        let r = (r_base * v) as u32;
                                        let g = (g_base * v) as u32;
                                        let b = (b_base * v) as u32;
                                        *col = (r << 16) | (g << 8) | b;
                                    }
                                }
                            });
                        }
                    }
                }
                RenderMethod::Merge(scope) => {
                    let mut scope = scope.clone();
                    scope.transform.x += self.transform.x;
                    scope.transform.y += self.transform.y;
                    scope.draw_buf(buf);
                }
            }
        }
    }
}

// Update or variable functions

impl RenderScope {
    pub fn set_transform(&mut self, transform: &Transform) {
        transform.use_dimensions(&mut self.transform);
        transform.use_position(self.parent_width, self.parent_height, &mut self.transform);
    }

    pub fn get_transform(&mut self) -> RawTransform {
        self.transform.clone()
    }

    pub fn update_size(&mut self) {
        self.max_size.0 = self.max_size.0.max(self.transform.width);
        self.max_size.1 = self.max_size.1.max(self.transform.height);
    }

    pub fn clear(&mut self) {
        self.render_stack.clear();
        self.transform.width = 0;
        self.transform.height = 0;
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.transform.width, self.transform.height)
    }

    pub fn get_max_size(&self) -> (usize, usize) {
        self.max_size
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
