pub mod mouse;
pub mod tick;
pub mod velocity;

use std::sync::Arc;

use minifb::Window;

use crate::{style::RawTransform, widget::Widget};

pub trait Extension {
    fn init(&mut self, _widgets: &Vec<Arc<Widget>>) {}
    fn before_render(
        &mut self,
        _widgets: &Vec<Arc<Widget>>,
        _transform: RawTransform,
        _win: &Window,
    ) {
    }
    fn render(&mut self, _widget: &Arc<Widget>, _transform: RawTransform, _win: &Window) {}
}
