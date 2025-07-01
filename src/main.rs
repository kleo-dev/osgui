use minifb::{Window, WindowOptions};
use osgui::{
    elements::Rect,
    extensions::velocity::{Velocity, VelocityExtension},
    Screen,
};

fn main() {
    let window = Window::new(
        "minifb example",
        500,
        500,
        WindowOptions {
            resize: false,
            ..Default::default()
        },
    )
    .unwrap();

    let mut app = Screen::new(window);
    app.extension(VelocityExtension);

    app.draw(Rect).component(Velocity(100, 0));

    app.run().unwrap();
}
