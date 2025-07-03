use std::sync::Arc;

use crate::{
    render::RenderScope,
    widget::{Element, Widget},
};

#[derive(Default)]
pub struct DivStyle {
    pub background_color: u32,
    pub corner_radius: usize,
}

#[derive(Default)]
pub struct Div {
    pub children: Vec<Arc<Widget>>,
    pub style: DivStyle,
}

impl Element for Div {
    fn render(&mut self, scope_parent: &mut crate::render::RenderScope) {
        let (w, h) = scope_parent.get_size_or_parent();
        scope_parent.draw_rect_rounded(
            0,
            0,
            w,
            h,
            self.style.corner_radius,
            self.style.background_color,
        );

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
            scope_parent.merge(scope.clone());
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
    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &Arc<Widget> {
        self.children.push(Arc::new(Widget::new(Box::new(element))));
        self.children.last().unwrap()
    }
}
