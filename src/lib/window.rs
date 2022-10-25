use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use super::rendering::State;

lazy_static! {
    pub static ref WINDOW_SIZE: RwLock<(f32, f32)> =
        RwLock::new((900.0, 600.0));
}

pub struct Window {
    event_loop: EventLoop<()>,
    pub state: Rc<RefCell<State>>,
}

impl Window {
    pub async fn new() -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let state = State::new(window).await;

        Self {
            event_loop,
            state: Rc::new(RefCell::new(state)),
        }
    }

    pub fn run_event_loop(
        self,
        state: Rc<RefCell<State>>,
        mut update: impl FnMut(),
    ) {
        self.event_loop.run(move |event, _, control_flow| {
            let mut state = state.borrow_mut();

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window.id() => {
                    if !state.input() {
                        // UPDATED!
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode:
                                            Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged {
                                new_inner_size,
                                ..
                            } => {
                                // new_inner_size is &&mut so w have to dereference it twice
                                state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id)
                    if window_id == state.window.id() =>
                {
                    log::info!("Redraw");
                    // match state.render() {
                    //     Ok(_) => {}
                    //     // Reconfigure the surface if it's lost or outdated
                    //     Err(
                    //         wgpu::SurfaceError::Lost
                    //         | wgpu::SurfaceError::Outdated,
                    //     ) => {
                    //         let size = state.size.clone();
                    //         state.resize(size);
                    //     }
                    //     // The system is out of memory, we should probably quit
                    //     Err(wgpu::SurfaceError::OutOfMemory) => {
                    //         *control_flow = ControlFlow::Exit
                    //     }

                    //     Err(wgpu::SurfaceError::Timeout) => {
                    //         log::warn!("Surface timeout")
                    //     }
                    // }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    state.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
