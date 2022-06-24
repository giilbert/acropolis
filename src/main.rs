mod core;
mod rendering;
use crate::core as giz_core;
use crate::core::ecs::{Component, Entity};
use crate::core::Application;
use rendering::components::mesh::Mesh;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut app = Application::new();

    let mesh = Rc::from(Mesh::new(&app));
    let entity = Rc::from(RefCell::new(Entity::new()));

    {
        let mut entity = entity.borrow_mut();
        entity.add_component(&mut app, mesh.clone());
    }

    app.start();
}
