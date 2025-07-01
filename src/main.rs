use minifb::{Window, WindowOptions};
use osgui::{extensions::velocity::VelocityExtension, style::Transform, Screen};

fn main() {
    let window = Window::new(
        "minifb example",
        200,
        200,
        WindowOptions {
            resize: false,
            ..Default::default()
        },
    )
    .unwrap();

    let mut app = Screen::new(window);
    app.extension(VelocityExtension);

    app.draw("test".to_string()).component(Transform::center());

    app.run().unwrap();
}
