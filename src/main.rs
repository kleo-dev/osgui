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

    d.draw("test".to_string());
    d.draw("abc".to_string());

    app.draw(d)
        .component(Transform::center().dimensions(150, 80));

    app.run().unwrap();
}
