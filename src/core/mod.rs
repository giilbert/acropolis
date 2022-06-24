pub mod ecs;
pub mod scene;
use crate::core::ecs::System;
use crate::rendering::RenderingSystem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Application<'a> {
    pub rendering: RenderingSystem<'a>,
}

impl std::fmt::Debug for Application<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("application")
    }
}

impl Application<'static> {
    pub fn new() -> Self {
        Application {
            rendering: RenderingSystem::new(),
        }
    }

    pub fn start(self) {
        let rc = Rc::new(RefCell::new(self));

        unsafe {
            let app = Rc::clone(&rc);
            let app = &mut *app.as_ptr();
            app.rendering.window.start(rc);
        }
    }
}
