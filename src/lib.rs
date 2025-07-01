use minifb::Window;

pub struct App {
    window: Window,
}

impl App {
    pub fn new(win: Window) -> Self {
        Self { window: win }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.render();
        }
    }

    fn render(&mut self) {
        let (w, h) = self.window.get_size();
        let mut buffer = vec![0; w * h];
        for y in 100..200 {
            for x in 150..300 {
                buffer[y * h + x] = 0xFF00FF;
            }
        }

        self.window.update_with_buffer(&buffer, w, h).unwrap();
    }
}
