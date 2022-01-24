use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, StartCause, WindowEvent}};


fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("starts").with_resizable(false).build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow|{
        match event {
            Event::WindowEvent{event: WindowEvent::CloseRequested, window_id} if window_id == window.id()=>*control_flow = ControlFlow::Exit,
            Event::NewEvents(StartCause::Init)=>*control_flow=ControlFlow::Poll,
            _ => {},

        }
    })
}
