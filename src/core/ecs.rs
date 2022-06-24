use crate::giz_core::Application;
use std::rc::Rc;

pub struct Entity<'a> {
    pub components: Vec<Rc<dyn Component<'a>>>,
}

impl<'a> Entity<'a> {
    pub fn new() -> Self {
        Entity { components: vec![] }
    }

    pub fn add_component(
        &mut self,
        app: &mut Application<'a>,
        component: Rc<dyn Component<'a>>,
    ) {
        component.on_add(app, component.clone());
        self.components.push(component.clone());
    }
}

pub trait Component<'a> {
    fn new(app: &Application) -> Self
    where
        Self: Sized;
    fn on_add(
        &self,
        app: &mut Application<'a>,
        component: Rc<dyn Component<'a>>,
    );
    fn add_entity(&mut self, entity: Rc<Entity>);
    fn on_entity_update(&self, app: &Application, entity: &Entity);
    fn update(&self, app: &mut Application);
}

pub trait System<'s> {
    fn new() -> Self;
    fn add_component(&mut self, component: Rc<dyn Component<'s>>);
    fn update(&mut self, app: &mut Application);
}
