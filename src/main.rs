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

    // milestones(&mut app);
    todos(&mut app);

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
    todo_box(app, Transform::center(), true, "Hello, World!");
}

fn todo_box(app: &mut Screen, transform: Transform, done: bool, text: &str) {
    let text = text.to_string();
    app.draw(custom(move |scope: &mut RenderScope| {
        scope.draw_rect_rounded(0, 2, 24, 24, 8, 0xffffff);
        scope.draw_rect_rounded(3, 5, 18, 18, 4, 0x010101);
        if done {
            let (mut x, mut y) = (5, 14);
            while x != 8 {
                scope.draw_rect_rounded(x, y, 2, 2, 4, 0xffffff);
                x += 1;
                y += 1;
            }

            while x != 17 && y != 7 {
                scope.draw_rect_rounded(x, y, 3, 2, 4, 0xffffff);
                x += 1;
                y -= 1;
            }
        }
        scope.draw_text(29, 0, 28.0, &text, 0xffffff);
    }))
    .component(transform.clone());
}
