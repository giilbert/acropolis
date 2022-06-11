use crate::giz_core::ecs::System;
mod window;

pub struct RenderingSystem {
    pub window: window::Window,
}

impl System for RenderingSystem {
    fn new() -> Self {
        return RenderingSystem {
            window: window::Window::new(),
        };
    }

    fn init(&mut self) {
        self.window.init();
    }
}
