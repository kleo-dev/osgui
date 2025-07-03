use once_cell::sync::Lazy;
use rusttype::{Font, Scale};

use crate::{
    style::{RawTransform, Transform},
    utils,
};

const FONT: &[u8] = include_bytes!("../assets/DejaVuSans.ttf");
static FONT_OBJ: Lazy<Font<'static>> = Lazy::new(|| Font::try_from_bytes(FONT).unwrap());

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
            max_size: (0, 0),
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, scale: f32, text: &str, color: u32) {
        if color == 0 {
            return;
        }
        self.render_stack
            .push(RenderMethod::Text(text.to_string(), x, y, scale, color));
        let (w, h) = utils::measure_text(&FONT_OBJ, text, Scale::uniform(scale));
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
        let stride = self.parent_width;
        let mut tmp = std::mem::take(&mut self.buffer);
        self.draw_buf(&mut tmp, stride);
        self.buffer = tmp;
    }

    pub fn draw_buf(&mut self, buf: &mut [u32], stride: usize) {
        let offset_x = self.transform.x;
        let offset_y = self.transform.y;

        for m in &mut self.render_stack {
            match m {
                /* ───────────── Rectangle ───────────── */
                RenderMethod::Rectangle(px, py, width, height, color) => {
                    let xs = offset_x + *px;
                    let xe = (xs + *width).min(self.parent_width);
                    let ys = offset_y + *py;
                    let ye = (ys + *height).min(self.parent_height);

                    for y in ys..ye {
                        let start = y * stride + xs;
                        let end = start + (xe - xs);
                        buf[start..end].fill(*color);
                    }
                }

                /* ─────────── Rounded Rectangle ─────────── */
                RenderMethod::RoundedRectangle(px, py, width, height, radius, color) => {
                    let r = *radius as i32;
                    let ww = *width as i32;
                    let hh = *height as i32;
                    let sx = (offset_x + *px) as i32;
                    let sy = (offset_y + *py) as i32;

                    for y in 0..hh {
                        for x in 0..ww {
                            // corners ⇒ circle test
                            let in_tl = x < r && y < r;
                            let in_tr = x >= ww - r && y < r;
                            let in_bl = x < r && y >= hh - r;
                            let in_br = x >= ww - r && y >= hh - r;

                            let ok = if in_tl {
                                (x - r + 1).pow(2) + (y - r + 1).pow(2) <= r.pow(2)
                            } else if in_tr {
                                (x - (ww - r)).pow(2) + (y - r + 1).pow(2) <= r.pow(2)
                            } else if in_bl {
                                (x - r + 1).pow(2) + (y - (hh - r)).pow(2) <= r.pow(2)
                            } else if in_br {
                                (x - (ww - r)).pow(2) + (y - (hh - r)).pow(2) <= r.pow(2)
                            } else {
                                true
                            };

                            if ok {
                                let gx = sx + x;
                                let gy = sy + y;
                                if gx >= 0
                                    && gx < self.parent_width as i32
                                    && gy >= 0
                                    && gy < self.parent_height as i32
                                {
                                    buf[(gy as usize) * stride + gx as usize] = *color;
                                }
                            }
                        }
                    }
                }

                /* ────────────────── Text ───────────────── */
                RenderMethod::Text(text, px, py, scale, base) => {
                    let scale = Scale::uniform(*scale);
                    let ascent = FONT_OBJ.v_metrics(scale).ascent;

                    let r0 = ((*base >> 16) & 0xFF) as f32;
                    let g0 = ((*base >> 8) & 0xFF) as f32;
                    let b0 = (*base & 0xFF) as f32;

                    for glyph in FONT_OBJ.layout(
                        text,
                        scale,
                        rusttype::point((offset_x + *px) as f32, (offset_y + *py) as f32 + ascent),
                    ) {
                        if let Some(bb) = glyph.pixel_bounding_box() {
                            glyph.draw(|gx, gy, v| {
                                if v < 0.5 {
                                    return;
                                }

                                let x = gx as i32 + bb.min.x;
                                let y = gy as i32 + bb.min.y;

                                let idx = (y as usize) * stride + x as usize;
                                if let Some(p) = buf.get_mut(idx) {
                                    let r = (r0 * v) as u32;
                                    let g = (g0 * v) as u32;
                                    let b = (b0 * v) as u32;
                                    *p = (r << 16) | (g << 8) | b;
                                }
                            });
                        }
                    }
                }

                /* ───────────────── Merge ───────────────── */
                RenderMethod::Merge(scope) => {
                    scope.transform.x += self.transform.x;
                    scope.transform.y += self.transform.y;
                    scope.draw_buf(buf, stride);
                }
            }
        }
    }
}

// Update or variable functions

impl RenderScope {
    pub fn resize_if_needed(&mut self, w: usize, h: usize) {
        if self.parent_width != w || self.parent_height != h {
            self.parent_width = w;
            self.parent_height = h;
            self.buffer.resize(w * h, 0);
        }
    }

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

    pub fn get_buffer1d(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn get_buffer(&self) -> Vec<&[u32]> {
        self.buffer.chunks(self.parent_width).collect()
    }

    pub fn get_buffer_mut(&mut self) -> Vec<&mut [u32]> {
        self.buffer.chunks_mut(self.parent_width).collect()
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.fill(0);
    }
}
