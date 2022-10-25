use crate::resources::rendering::StateResource;

use super::window::Window;
use bevy_ecs::prelude::*;

pub struct Application {
    window: Window,
    pub world: World,
    runtime_schedule: Schedule,
}

impl Application {
    pub fn new() -> Application {
        let window = pollster::block_on(Window::new());

        let mut world = World::new();
        world.insert_non_send_resource(StateResource(window.state.clone()));

        let mut runtime_schedule = Schedule::default();
        runtime_schedule.add_stage("render", SystemStage::parallel());

        Application {
            window,
            world,
            runtime_schedule,
        }
    }

    pub fn run(mut self) {
        let state = self.window.state.clone();
        self.window.run_event_loop(state, move || {
            self.runtime_schedule.run(&mut self.world);
        });
    }
}
