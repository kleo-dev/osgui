use minifb::{Window, WindowOptions};
use osgui::App;

fn main() {
    let window = Window::new("minifb example", 500, 500, WindowOptions::default()).unwrap();

    let mut app = App::new(window);

    app.run();
}
