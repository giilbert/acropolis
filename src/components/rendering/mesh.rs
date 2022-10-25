use std::rc::Rc;

use bevy_ecs::prelude::*;

use crate::lib::rendering::{Material, Vertex};

#[derive(Component)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    vao: VertexArray,
}

impl Mesh {
    pub fn new(
        gl: Rc<Context>,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Mesh {
        Mesh {
            vao,
            vertex_buffer,
            index_buffer,
            vertices,
            indices,
        }
    }

    pub fn bind_material(&self, gl: &Context, material: &Material) {}

    pub fn draw(&self, gl: &Context) {}
}
