use std::rc::Rc;

use bevy_ecs::prelude::*;
use glow::{Buffer, Context, HasContext, VertexArray};

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
        let (vao, vertex_buffer, index_buffer) = unsafe {
            let vao = gl.create_vertex_array().expect("Error creating VAO");
            gl.bind_vertex_array(Some(vao));

            let vertex_buffer =
                gl.create_buffer().expect("Error creating vertex buffer");
            let index_buffer =
                gl.create_buffer().expect("Error creating index buffer");

            // convert and store the data in vertices into vertex buffer
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices),
                glow::STATIC_DRAW,
            );

            (vao, vertex_buffer, index_buffer)
        };

        Mesh {
            vao,
            vertex_buffer,
            index_buffer,
            vertices,
            indices,
        }
    }

    pub fn bind_material(&self, gl: &Context, material: &Material) {
        unsafe {
            gl.use_program(Some(material.program));
            gl.bind_vertex_array(Some(self.vao));

            let vertex_position_attribute_location =
                material.attributes[0].location;

            gl.vertex_attrib_pointer_f32(
                vertex_position_attribute_location,
                3,
                glow::FLOAT,
                false,
                0,
                0,
            );
            gl.enable_vertex_attrib_array(vertex_position_attribute_location);
        }
    }

    pub fn draw(&self, gl: &Context) {
        unsafe {
            // log::info!("drawing mesh");
            gl.bind_vertex_array(Some(self.vao));
            // gl.draw_arrays(glow::TRIANGLES, 0, 3);
            gl.clear_color(1.0, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.draw_elements(
                glow::TRIANGLES,
                self.indices.len().try_into().unwrap(),
                glow::UNSIGNED_INT,
                0,
            );
        }
    }
}
