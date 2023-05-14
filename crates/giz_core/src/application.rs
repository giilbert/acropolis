use crate::Stage;
use bevy_ecs::prelude::*;

use crate::Plugin;

pub struct Application {
    pub world: Box<World>,
    pub runner: Box<dyn FnOnce(Application) -> ()>,
    pub init_schedule: Schedule,
    pub runtime_schedule: Schedule,
}

pub fn noop_runner(_app: Application) {}

impl Application {
    pub fn new() -> Application {
        let world = World::new();
        let init_schedule = Schedule::default();
        let runtime_schedule = Schedule::default()
            .with_stage(Stage::Scripting, SystemStage::parallel())
            .with_stage(Stage::Update, SystemStage::parallel())
            .with_stage(Stage::Render, SystemStage::parallel());

        Application {
            world: Box::new(world),
            runner: Box::new(noop_runner),
            runtime_schedule,
            init_schedule,
        }
    }

    pub fn run(mut self) {
        self.init_schedule.run_once(&mut self.world);
        let runner = std::mem::replace(&mut self.runner, Box::new(noop_runner));
        runner(self);
    }

    pub fn with_plugin(mut self, mut plugin: impl Plugin) -> Self {
        plugin.build(&mut self);
        self
    }
}
