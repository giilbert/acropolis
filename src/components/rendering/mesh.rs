use std::rc::Rc;

use bevy_ecs::prelude::*;

use crate::lib::rendering::Material;

#[derive(Component)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(
        gl,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Self {
    }
}
