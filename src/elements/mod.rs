use crate::widget::Element;

pub struct Rect;

impl Element for Rect {
    fn render(&mut self, scope: &mut crate::render::RenderScope) {
        let (w, h) = scope.get_size();
        scope.draw_rect(0, 0, w, h, 0xffffff);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Element for String {
    fn render(&mut self, scope: &mut crate::render::RenderScope) {
        scope.draw_text(0, 0, 40.0, self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
