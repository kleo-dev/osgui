use crate::widget::Element;

pub struct Rect;

impl Element for Rect {
    fn render(&mut self, scope: &mut crate::render::RenderScope) {
        scope.draw_rect(300, 300, 0xffffff);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
