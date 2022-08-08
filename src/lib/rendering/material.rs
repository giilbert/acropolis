use std::rc::Rc;

use bevy_ecs::prelude::*;
use glow::{Context, HasContext, Program, Shader};
use uuid::Uuid;

use crate::components::rendering::Mesh;
pub struct Material {
    pub id: Uuid,
    pub program: Program,
    pub attributes: Vec<Attribute>,
    pub uniforms: Vec<Uniform>,
    pub entities: Vec<Entity>,
    gl: Rc<Context>,
}

impl Material {
    pub fn new(
        gl: Rc<Context>,
        vertex_shader_source: String,
        fragment_shader_source: String,
        attributes: Vec<impl Into<String>>,
        uniforms: Vec<impl Into<String>>,
    ) -> Result<Material, String> {
        let vertex_shader =
            compile_shader(&gl, glow::VERTEX_SHADER, &vertex_shader_source)?;
        let fragment_shader = compile_shader(
            &gl,
            glow::FRAGMENT_SHADER,
            &fragment_shader_source,
        )?;
        let program = link_program(&gl, vertex_shader, fragment_shader)?;

        unsafe { gl.use_program(Some(program)) };

        let uniforms = get_uniforms(&gl, program, uniforms)?;
        let attributes = get_attributes(&gl, program, attributes)?;

        Ok(Material {
            id: Uuid::new_v4(),
            program,
            uniforms,
            attributes,
            entities: vec![],
            gl,
        })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    pub fn bind_mesh(&mut self, world: &mut World, entity_id: Entity) {
        let component = world
            .get::<Mesh>(entity_id)
            .expect("Unable to bind an entity without a Mesh component");

        component.bind_material(self.gl.as_ref(), self);

        self.entities.push(entity_id);
    }
}

fn compile_shader(
    gl: &Rc<Context>,
    shader_type: u32,
    shader_source: &str,
) -> Result<Shader, String> {
    let shader = unsafe {
        let shader = gl.create_shader(shader_type)?;
        gl.shader_source(shader, shader_source);
        gl.compile_shader(shader);
        shader
    };
    let info = unsafe { gl.get_shader_info_log(shader) };
    if info.len() > 0 {
        return Err(info);
    }

    Ok(shader)
}

fn link_program(
    gl: &Context,
    vertex_shader: Shader,
    fragment_shader: Shader,
) -> Result<Program, String> {
    let program = unsafe {
        let program = gl.create_program()?;
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);
        program
    };

    let info = unsafe { gl.get_program_info_log(program) };
    if info.len() > 0 {
        return Err(info);
    }

    unsafe {
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);
    }

    Ok(program)
}

pub struct Attribute {
    pub name: String,
    pub location: u32,
}

fn get_attributes(
    gl: &Context,
    program: Program,
    names: Vec<impl Into<String>>,
) -> Result<Vec<Attribute>, String> {
    let mut attributes = Vec::new();

    for name in names {
        let name = name.into();
        let name = name.as_str();
        let location = unsafe { gl.get_attrib_location(program, name) };

        match location {
            None => return Err(format!("Attribute {} not found", name)),
            Some(location) => attributes.push(Attribute {
                name: name.to_string(),
                location,
            }),
        }
    }

    Ok(attributes)
}

pub struct Uniform {
    pub name: String,
    pub location: u32,
}

fn get_uniforms(
    gl: &Context,
    program: Program,
    names: Vec<impl Into<String>>,
) -> Result<Vec<Uniform>, String> {
    let mut uniforms = Vec::new();

    for name in names {
        let name = name.into();
        let name = name.as_str();
        let location = unsafe { gl.get_attrib_location(program, name) };

        match location {
            None => return Err(format!("Uniform {} not found", name)),
            Some(location) => uniforms.push(Uniform {
                name: name.to_string(),
                location,
            }),
        }
    }

    Ok(uniforms)
}

impl Drop for Material {
    fn drop(&mut self) {
        unsafe { self.gl.delete_program(self.program) }
    }
}
