use crate::render::RenderScope;

pub trait Element {
    fn render(&mut self, scope: &mut RenderScope);
}
