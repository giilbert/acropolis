use std::rc::Rc;

pub struct Window {
    event_loop: glutin::event_loop::EventLoop<()>,
    window:
        glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    pub gl: Rc<glow::Context>,
}

impl Window {
    pub fn new() -> Self {
        use glutin::dpi::LogicalSize;

        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("giz")
            .with_inner_size::<LogicalSize<i32>>(LogicalSize::new(1024, 768));

        let (window, gl) = unsafe {
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .expect("unable to create window")
                .make_current()
                .expect("could not make opengl context current");

            let gl: glow::Context = glow::Context::from_loader_function(|s| {
                window.get_proc_address(s) as *const _
            });

            (window, gl)
        };

        Self {
            event_loop,
            window,
            gl: Rc::new(gl),
        }
    }

    pub fn run_event_loop(self, mut update: Box<dyn FnMut()>) {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    update();
                    self.window.swap_buffers().unwrap();
                }
                Event::RedrawRequested(_) => {}
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {}
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
