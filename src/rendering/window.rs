pub struct Window {}

impl Window {
    pub fn init(&self) {
        use glium::glutin;

        // 1. The **winit::EventsLoop** for handling events.
        let event_loop = glium::glutin::event_loop::EventLoop::new();
        // 2. Parameters for building the Window.
        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(
                1024.0, 768.0,
            ))
            .with_title("Hello world");
        // 3. Parameters for building the OpenGL context.
        let cb = glium::glutin::ContextBuilder::new();
        // 4. Build the Display with the given window and OpenGL context parameters and register the
        //    window with the events_loop.
        #[allow(unused)]
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        // let proxy = event_loop.create_proxy();

        event_loop.run(move |ev, _, control_flow| {
            #[allow(unused)]
            let next_frame_time = std::time::Instant::now()
                + std::time::Duration::from_nanos(16_666_667);

            *control_flow =
                glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            match ev {
                glutin::event::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow =
                                glutin::event_loop::ControlFlow::Exit;
                            return;
                        }
                        _ => return,
                    }
                }
                _ => (),
            }
        });
    }

    pub fn new() -> Self {
        return Window {};
    }
}
