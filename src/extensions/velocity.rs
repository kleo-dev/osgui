use std::sync::Arc;

use crate::{component, extensions::Extension, style::Transform, widget::Widget};

pub struct VelocityExtension;

impl VelocityExtension {
    fn apply_velocity(ticks: usize, velocity: i32, x: &mut i32) {
        if velocity.abs() != 0 && ticks as i32 % (1000 / velocity.abs()) == 0 {
            if velocity > 0 {
                *x += 1;
            } else if velocity < 0 {
                *x -= 1;
            }
        }
    }
}

impl VelocityExtension {
    fn apply_velocity_xy(ticks: usize, widget: &Arc<Widget>) {
        if let Some(velocity) = widget.get::<Velocity>() {
            if let Some(mut t) = widget.get::<Transform>() {
                Self::apply_velocity(ticks, velocity.0, &mut t.mx);
                Self::apply_velocity(ticks, velocity.1, &mut t.my);
                widget.set_component(t);
            }
        }
    }
}

impl Extension for VelocityExtension {
    fn init(&mut self, widgets: &Vec<Arc<Widget>>) {
        std::thread::spawn({
            let widgets = widgets.clone();
            move || {
                let mut tick = 0;
                loop {
                    for widget in &widgets {
                        Self::apply_velocity_xy(tick, widget);
                    }
                    if tick > 1000 {
                        tick = 0;
                    } else {
                        tick += 1;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        });
    }
}

component!(Velocity(pub i32, pub i32));
