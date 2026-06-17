use winit::event_loop::EventLoop;

use crate::app::App;

mod gb;
mod state;
mod app;
fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
    gb::emulate("test/test.gb").unwrap();
}
