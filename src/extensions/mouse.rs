use std::sync::Arc;

use minifb::{MouseButton, Window};

use crate::{component, extensions::Extension, style::RawTransform, widget::Widget};

pub struct MouseExtension(bool);

impl Extension for MouseExtension {
    fn render(&mut self, widget: &Arc<Widget>, transform: RawTransform, win: &Window) {
        if win.get_mouse_down(MouseButton::Left) {
            if !self.0 {
                self.0 = true;
                if let Some((mx, my)) = win.get_mouse_pos(minifb::MouseMode::Discard) {
                    let (mx, my) = (mx as usize, my as usize);
                    if mx >= transform.x
                        && my >= transform.y
                        && mx <= transform.x + transform.width
                        && mx <= transform.y + transform.height
                    {
                        std::thread::spawn({
                            let widget = widget.clone();
                            move || {
                                if let Some(on_click) = widget.get::<OnClick>() {
                                    (on_click.0)(&widget)
                                }
                            }
                        });
                    }
                }
            }
        } else {
            self.0 = false;
        }
    }
}

impl MouseExtension {
    pub fn new() -> Self {
        MouseExtension(false)
    }
}

component!(OnClick(pub fn(&Arc<Widget>)));
