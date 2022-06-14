use crate::giz_core::Application;
use std::rc::Rc;

pub struct Entity<'a> {
    pub components: Vec<Box<dyn Component<'a>>>,
}

impl Entity<'_> {
    pub fn new() -> Self {
        Entity { components: vec![] }
    }
    fn add_component() {}
}

pub trait Component<'a> {
    fn new(app: &Application) -> Self
    where
        Self: Sized;
    fn on_entity_update(&self, app: &Application, entity: &Entity);
    fn update(&self, app: &Application, entity: &Entity);
}

pub trait System<'s> {
    fn new() -> Self;
    fn add_component(&mut self, component: Rc<dyn Component<'s>>);
}
