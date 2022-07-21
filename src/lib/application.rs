use crate::lib::window::Window;
use crate::resources::*;
use bevy_ecs::{
    prelude::World,
    schedule::{Schedule, SystemStage},
};
use glow::HasContext;

pub struct Application {
    window: Window,
    world: World,
    schedule: Schedule,
}

impl Application {
    pub fn new() -> Application {
        let window = Window::new();

        let mut world = World::default();
        world.insert_non_send_resource(
            rendering::RenderingResource {
                gl: window.gl.clone(),
            },
        );

        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::parallel());

        unsafe {
            let gl = window.gl.clone();
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }

        Application {
            window,
            world,
            schedule,
        }
    }

    pub fn run(mut self) {
        let update = move || {
            self.schedule.run_once(&mut self.world);
        };

        self.window.run_event_loop(Box::new(update));
    }
}
