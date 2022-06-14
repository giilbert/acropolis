mod core;
mod rendering;
use crate::core as giz_core;
use crate::core::ecs::{Component, Entity, System};
use rendering::components::mesh::Mesh;
use std::rc::Rc;

fn main() {
    let mut app = giz_core::Application::new();
    let mesh = Rc::from(Mesh::new(&app));
    let entity = Entity::new();
    app.rendering.add_component(mesh.clone());
    mesh.update(&app, &entity);
    app.start();
}
