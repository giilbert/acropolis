use lazy_static::lazy_static;
use std::{cell::RefCell, rc::Rc, sync::RwLock};
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
    pub state: State,
}

impl Window {
    pub async fn new() -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let state = State::new(window).await;

        Self { event_loop, state }
    }

    pub fn run_event_loop(
        self,
        state: State,
        mut update: impl FnMut() + 'static,
    ) {
        self.event_loop.run(move |event, _, control_flow| {
            let mut state = state.lock();

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
                    drop(state);
                    update();
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
