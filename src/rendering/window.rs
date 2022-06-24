use crate::core::ecs::System;
use crate::giz_core::Application;
use glium::glutin::event_loop::EventLoop;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Window {
    pub display: glium::Display,
    event_loop: Option<EventLoop<()>>,
}

impl Window {
    pub fn new() -> Self {
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
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        Self {
            display,
            event_loop: Some(event_loop),
        }
    }

    pub fn start(&mut self, app_rc_cell: Rc<RefCell<Application<'static>>>) {
        use glium::glutin;

        let app_rc = app_rc_cell.clone();

        self.event_loop
            .take()
            .unwrap()
            .run(move |ev, _, control_flow| {
                let next_frame_time = std::time::Instant::now()
                    + std::time::Duration::from_nanos(16_666_667);

                *control_flow =
                    glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

                let app_param = unsafe { &mut *app_rc.as_ptr() };
                let mut app = app_rc.borrow_mut();
                app.rendering.update(app_param);

                match ev {
                    glutin::event::Event::WindowEvent { event, .. } => {
                        match event {
                            glutin::event::WindowEvent::CloseRequested => {
                                *control_flow =
                                    glutin::event_loop::ControlFlow::Exit;
                                return;
                            }

                            glutin::event::WindowEvent::KeyboardInput {
                                device_id: _,
                                input,
                                is_synthetic: _,
                            } => {
                                if input.virtual_keycode.is_none() {
                                    return;
                                }

                                let keycode = input.virtual_keycode.unwrap();

                                if keycode
                                    == glutin::event::VirtualKeyCode::Escape
                                {
                                    *control_flow =
                                        glutin::event_loop::ControlFlow::Exit;
                                    return;
                                }
                            }

                            _ => return,
                        }
                    }
                    _ => (),
                }
            });
    }
}
