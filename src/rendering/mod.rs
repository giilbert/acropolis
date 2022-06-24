use crate::giz_core::ecs::{Component, System};
use crate::giz_core::Application;
use std::rc::Rc;
pub mod components;
pub mod window;

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

    fn update(&mut self, app: &mut Application) {
        for component in self.components.iter_mut() {
            component.update(app);
        }
    }
}