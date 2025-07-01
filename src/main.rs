use minifb::{Window, WindowOptions};
use osgui::{
    elements::Rect,
    extensions::velocity::{Velocity, VelocityExtension},
    style::Transform,
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

    app.draw(Rect)
        .component(Transform::new().dimensions(40, 40))
        .component(Velocity(500, 0));

    app.run().unwrap();
}
