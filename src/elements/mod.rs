use crate::widget::Element;

pub struct Rect;

impl Element for Rect {
    fn render(&mut self, scope: &mut crate::render::RenderScope) {
        scope.draw_rect();
    }
}
