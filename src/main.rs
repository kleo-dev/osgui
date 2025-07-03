use minifb::{Window, WindowOptions};
use osgui::{
    elements::div::{Div, DivStyle},
    extensions::mouse::{MouseExtension, OnClick},
    style::Transform,
    Screen,
};

fn main() {
    let window = Window::new(
        "minifb example",
        1920,
        1080,
        WindowOptions {
            resize: false,
            ..Default::default()
        },
    )
    .unwrap();

    let mut app = Screen::new(window);
    app.extension(MouseExtension::new());

    milestones(&mut app);

    app.run().unwrap();
}

fn milestones(app: &mut Screen) {
    for i in 0..4 {
        let mut milestone = Div {
            style: DivStyle {
                background_color: 0x1D1D1D,
                corner_radius: 24,
                ..Default::default()
            },
            ..Default::default()
        };

        milestone
            .draw(format!("My Milestone"))
            .component(Transform::center());

        app.draw(milestone)
            .component(Transform::new().dimensions(873, 194).pos(
                osgui::style::Position::Center,
                osgui::style::Position::Const(145 + ((194 + 30) * i)),
            ))
            .component(OnClick(|_| println!("Clicked!")));
    }
}
