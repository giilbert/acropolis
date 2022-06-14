use crate::giz_core::ecs::Component;
use crate::giz_core::ecs::System;
use std::rc::Rc;
pub mod components;
mod window;

pub struct RenderingSystem<'a> {
    pub window: window::Window,
    components: Vec<Rc<dyn Component<'a>>>,
}

impl<'s> System<'s> for RenderingSystem<'s> {
    fn new() -> Self {
        RenderingSystem {
            window: window::Window::new(),
            components: vec![],
        }
    }

    fn add_component(&mut self, component: Rc<dyn Component<'s>>) {
        self.components.push(component.clone());
    }
}
