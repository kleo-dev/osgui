mod app;

pub struct Window {
    width: f64,
    height: f64,
}

impl Window {
    pub fn new() -> Window {
        Window {
            width: 1000.0,
            height: 650.0,
        }
    }

    pub fn run(&self) {
        app::run(self.width, self.height);
    }
}
