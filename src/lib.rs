use minifb::Window;
use std::sync::{Arc, Mutex};

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
    pub fn new(win: Window) -> Self {
        Self {
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
        for elem in &self.widgets {
            elem.component(Transform::new());
        }

        for ext in &self.extensions {
            ext.lock().unwrap().init(&self.widgets);
        }

        let (w, h) = self.window.get_size();
        let mut scope = RenderScope::new(w, h);

        let mut last_fps_check = std::time::Instant::now();
        let mut frame_count = 0;

        while self.window.is_open() {
            self.render(&mut scope);
            frame_count += 1;

            // Print FPS every second
            if last_fps_check.elapsed() >= std::time::Duration::from_secs(1) {
                println!("{}", frame_count);
                frame_count = 0;
                last_fps_check = std::time::Instant::now();
            }
        }

        Ok(())
    }

    fn render(&mut self, scope: &mut RenderScope) {
        let (w, h) = self.window.get_size();
        scope.resize_if_needed(w, h);

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

            elem.0.lock().unwrap().render(scope);

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
            .update_with_buffer(scope.get_buffer1d(), w, h)
            .unwrap();
        scope.clear_buffer();
    }
}
