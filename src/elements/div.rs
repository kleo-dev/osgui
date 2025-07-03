use std::sync::Arc;

use crate::{
    render::RenderScope,
    widget::{Element, Widget},
};

pub struct DivStyle {
    background_color: u32,
}

pub struct Div {
    children: Vec<Arc<Widget>>,
    style: DivStyle,
}

impl Element for Div {
    fn render(&mut self, scope_parent: &mut crate::render::RenderScope) {
        let (w, h) = scope_parent.get_size_or_parent();
        scope_parent.draw_rect(0, 0, w, h, self.style.background_color);
        let mut scope = RenderScope::new(w, h);

        for elem in &self.children {
            scope.clear();
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }

            elem.0.lock().unwrap().render(&mut scope);

            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }
            scope_parent.merge(scope.clone(), 0, 0);
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
        Self {
            children: Vec::new(),
            style: DivStyle {
                background_color: 0xF00000,
            },
        }
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &Arc<Widget> {
        self.children.push(Arc::new(Widget::new(Box::new(element))));
        self.children.last().unwrap()
    }
}
