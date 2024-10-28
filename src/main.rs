use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
fn main() {
    #[derive(Default)]
    struct App {
        window: Option<Window>,
    }
    impl ApplicationHandler for App {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
            println!("Resumed");
        }

        #[allow(unused_variables)]
        fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent,) {
            match event {
                WindowEvent::CloseRequested => {
                    println!("Close requested");
                    event_loop.exit();
                },
                WindowEvent::RedrawRequested => {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    self.window.as_ref().unwrap().request_redraw();
                    println!("Redraw requested");
                }
                _ => {}
            }
        }
    }

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}