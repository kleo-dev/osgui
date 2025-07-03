use std::sync::{Arc, Mutex};

use minifb::Window;

use crate::{
    extensions::Extension,
    render::RenderScope,
    style::Transform,
    widget::{Element, Widget},
};

pub mod elements;
pub mod extensions;
pub mod macros;
pub mod render;
pub mod style;
pub mod utils;
pub mod widget;

pub struct Screen {
    window: Window,
    pub widgets: Vec<Arc<Widget>>,
    extensions: Vec<Arc<Mutex<Box<dyn Extension>>>>,
}

impl Screen {
    pub fn new(win: Window) -> Screen {
        Screen {
            window: win,
            widgets: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn draw<E: Element + 'static>(&mut self, element: E) -> &Arc<Widget> {
        self.widgets.push(Arc::new(Widget::new(Box::new(element))));
        self.widgets.last().unwrap()
    }

    pub fn extension<E: Extension + 'static>(&mut self, ext: E) {
        self.extensions.push(Arc::new(Mutex::new(Box::new(ext))));
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        for elem in &mut self.widgets {
            elem.component(Transform::new());
        }

        for ext in &self.extensions {
            ext.lock().unwrap().init(&self.widgets);
        }

        while self.window.is_open() {
            self.render();
            std::thread::sleep(std::time::Duration::from_millis(28));
        }

        Ok(())
    }

    fn render(&mut self) {
        let (w, h) = self.window.get_size();
        let mut scope = RenderScope::new(w, h);

        for ext in &self.extensions {
            ext.lock()
                .unwrap()
                .before_render(&self.widgets, scope.get_transform(), &self.window);
        }

        for elem in &self.widgets {
            scope.clear();
            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }

            elem.0.lock().unwrap().render(&mut scope);

            if let Some(t) = elem.get() {
                scope.set_transform(&t);
            }

            for ext in &self.extensions {
                ext.lock()
                    .unwrap()
                    .render(elem, scope.get_transform(), &self.window);
            }

            scope.draw();
        }

        self.window
            .update_with_buffer(&scope.get_buffer1d(), w, h)
            .unwrap();
    }
}
