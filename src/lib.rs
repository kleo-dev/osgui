pub mod elements;
pub mod render;
pub mod widget;

use std::sync::{Arc, Mutex};

use minifb::Window;

use crate::{render::RenderScope, widget::Element};

pub struct App {
    window: Window,
    elements: Vec<Arc<Mutex<Box<dyn Element>>>>,
}

impl App {
    pub fn new(win: Window) -> Self {
        Self {
            window: win,
            elements: Vec::new(),
        }
    }

    pub fn element<E: Element + 'static>(&mut self, e: E) {
        self.elements.push(Arc::new(Mutex::new(Box::new(e))));
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.render();
        }
    }

    fn render(&mut self) {
        let (w, h) = self.window.get_size();
        let mut scope = RenderScope::new(w, h);

        for e in &self.elements {
            e.lock().unwrap().render(&mut scope);
        }

        self.window
            .update_with_buffer(&scope.get_buffer(), w, h)
            .unwrap();
    }
}
