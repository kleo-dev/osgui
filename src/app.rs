use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

struct App {
    window: Option<Window>,
    attr: WindowAttributes,
}

impl App {
    pub fn new(attr: WindowAttributes) -> App {
        App { window: None, attr }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(self.attr.clone()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn run(width: f64, height: f64) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut w = WindowAttributes::default();
    w.inner_size = Some(winit::dpi::Size::Logical(LogicalSize::new(width, height)));
    let mut app = App::new(w);
    event_loop.run_app(&mut app).unwrap();
}
