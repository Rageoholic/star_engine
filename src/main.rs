mod render_context;
use render_context::RenderContext;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("starts")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let _render_context = RenderContext::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        Event::NewEvents(StartCause::Init) => *control_flow = ControlFlow::Poll,
        _ => {}
    })
}
