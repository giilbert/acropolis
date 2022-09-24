use lazy_static::lazy_static;
use std::rc::Rc;
use std::sync::RwLock;

lazy_static! {
    pub static ref WINDOW_SIZE: RwLock<(f32, f32)> =
        RwLock::new((900.0, 600.0));
}

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
            .with_inner_size::<LogicalSize<i32>>(LogicalSize::new(900, 600));

        let (window, gl) = unsafe {
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .with_double_buffer(Some(true))
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
            *control_flow = ControlFlow::Poll;
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    self.window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    update();
                    self.window.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        self.window.resize(*physical_size);
                        *WINDOW_SIZE.write().unwrap() = (*physical_size).into();
                    }
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
