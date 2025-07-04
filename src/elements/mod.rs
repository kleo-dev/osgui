pub mod div;

use crate::widget::Element;

pub struct Rect(pub u32);

impl Element for Rect {
    fn render(&mut self, scope: &mut crate::render::RenderScope) {
        let (w, h) = scope.get_size();
        scope.draw_rect(0, 0, w, h, self.0);
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
        scope.draw_text(0, 0, 40.0, self, 0xffffff);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Element for (String, f32) {
    fn render(&mut self, scope: &mut crate::render::RenderScope) {
        scope.draw_text(0, 0, 40.0 * self.1, &self.0, 0xffffff);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Element for Box<dyn FnMut(&mut crate::render::RenderScope) + Send + Sync> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(&mut self, scope: &mut crate::render::RenderScope) {
        self(scope)
    }
}

impl Element for Box<dyn Fn(&mut crate::render::RenderScope) + Send + Sync> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(&mut self, scope: &mut crate::render::RenderScope) {
        self(scope)
    }
}

pub fn custom<F: FnMut(&mut crate::render::RenderScope) + Send + Sync + 'static>(
    f: F,
) -> Box<dyn FnMut(&mut crate::render::RenderScope) + Send + Sync> {
    Box::new(f)
}
