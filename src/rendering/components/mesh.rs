use crate::giz_core::{ecs::Component, ecs::Entity, Application};
use glium::{implement_vertex, Surface};
use glium::{index, VertexBuffer};
use std::rc::Rc;
use std::cell::Ref;

pub struct Mesh {
}

impl<'a> Component<'a> for Mesh {
    fn new(app: &Application) -> Self {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            // TODO: other vertex fields
        }
        implement_vertex!(Vertex, position);

        let vertex1 = Vertex {
            position: [-0.5, -0.5],
        };
        let vertex2 = Vertex {
            position: [0.0, 0.5],
        };
        let vertex3 = Vertex {
            position: [0.5, -0.25],
        };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer =
            VertexBuffer::new(&app.rendering.window.display, &shape).unwrap();
        let indices =
            index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        Mesh {}
    }

    fn on_add(
        &self,
        app: &mut Application<'a>,
        component: Rc<dyn Component<'a>>,
    ) {
        let clone = component.clone();
        app.rendering.components.push(clone);
    }

    fn on_entity_update(&self, app: &Application, entity: &Entity) {
        let mut target = app.rendering.window.display.draw();
    }

    fn update(&self, app: &mut Application) {
    }

    fn add_entity(&mut self, entity: Rc<Entity>) {
        todo!()
    }
}
