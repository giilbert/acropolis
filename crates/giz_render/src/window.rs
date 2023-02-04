use crate::State;
use lazy_static::lazy_static;
use std::{sync::RwLock, time::Instant};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// use super::rendering::State;

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

    pub fn run_event_loop(self, mut update: impl FnMut() + 'static) {
        // every 2 seconds at 60fps
        const PROFILE_NUM_FRAMES: i32 = 2 * 60;

        let mut frames = 0;
        let mut last_updated = Instant::now();

        let mut mspf_acc = 0;

        self.event_loop.run(move |event, _, control_flow| {
            let mut state = self.state.lock();

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window.id() => {
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
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(key_code),
                                    state: key_state,
                                    ..
                                },
                            ..
                        } => {
                            if *key_state == ElementState::Pressed {
                                state.keys.insert(*key_code);
                            } else {
                                state.keys.remove(key_code);
                            }

                            // log::info!(
                            //     "{} {:?}",
                            //     serde_json::to_string(key_code).unwrap(),
                            //     state.keys
                            // );
                        }
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
                Event::RedrawRequested(window_id)
                    if window_id == state.window.id() =>
                {
                    let elapsed = last_updated.elapsed().as_millis();
                    last_updated = Instant::now();

                    drop(state);
                    update();

                    mspf_acc += elapsed;
                    frames += 1;

                    if frames == PROFILE_NUM_FRAMES {
                        log::info!(
                            "mspf avg: {:.03}ms ({} frames)",
                            mspf_acc as f32 / PROFILE_NUM_FRAMES as f32,
                            PROFILE_NUM_FRAMES
                        );

                        frames = 0;
                        mspf_acc = 0;
                    }
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
