use minifb::{Window, WindowOptions};
use osgui::{
    elements::div::Div, extensions::velocity::VelocityExtension, style::Transform, Screen,
};

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

    let mut d = Div::new();

    d.draw("test".to_string()).component(Transform::center());
    d.draw("abc".to_string())
        .component(Transform::center().margin(0, 25));

    app.draw(d)
        .component(Transform::center().dimensions(150, 150));

    app.run().unwrap();
}
