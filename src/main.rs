use minifb::{Window, WindowOptions};
use osgui::{
    elements::{
        custom,
        div::{Div, DivStyle},
    },
    extensions::{
        mouse::{MouseExtension, OnClick},
        velocity::{Velocity, VelocityExtension},
    },
    render::RenderScope,
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
    app.extension(VelocityExtension);

    milestones(&mut app);
    // todos(&mut app);

    app.run().unwrap();
}

fn milestones(app: &mut Screen) {
    app.draw(format!("My Project"))
        .component(Transform::center().top(20));

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
            .component(OnClick(|_| println!("Clicked!")))
            .component(Velocity(300, 0));
    }
}

fn todos(app: &mut Screen) {
    for row in 0..10 {
        let mut task = Div {
            style: DivStyle {
                background_color: 0x191919,
                corner_radius: 13,
            },
            ..Default::default()
        };

        task.draw(custom(plus)).component(Transform::new());

        app.draw(task)
            .component(Transform::new().dimensions(550, 80).pos(
                osgui::style::Position::Center,
                osgui::style::Position::Const(90 + (100 * row)),
            ));
    }
}

fn plus(scope: &mut RenderScope) {
    scope.draw_rect(0, 12, 26, 3, 0xffffff);
    scope.draw_rect(12, 0, 3, 26, 0xffffff);
}
