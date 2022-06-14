pub mod ecs;
pub mod scene;
use crate::core::ecs::System;
use crate::rendering::RenderingSystem;

pub struct Application<'a> {
    pub rendering: RenderingSystem<'a>,
}

impl Application<'_> {
    pub fn new() -> Self {
        Application {
            rendering: RenderingSystem::new(),
        }
    }

    pub fn start(self) {
        // starts window event loop (blocks main thread)
        self.rendering.window.start();
    }
}
