use std::sync::Arc;

use crate::{
    render::RenderScope,
    widget::{Element, Widget},
};

pub struct Div(Vec<Arc<Widget>>);

impl Element for Div {
    fn render(&mut self, scope_parent: &mut crate::render::RenderScope) {
        let (w, h) = scope_parent.get_size_or_parent();
        let mut scope = RenderScope::new(w, h);

        for elem in &self.0 {
            scope.clear();

            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }

            elem.0.lock().unwrap().render(&mut scope);

            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }

            scope.draw_buf(scope_parent.get_buffer_mut());
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Div {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &Arc<Widget> {
        self.0.push(Arc::new(Widget::new(Box::new(element))));
        self.0.last().unwrap()
    }
}
