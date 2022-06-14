use crate::giz_core::{ecs::Component, ecs::Entity, Application};
use glium::implement_vertex;
use glium::{index, VertexBuffer};

pub struct Mesh {}

impl Component<'_> for Mesh {
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

    fn on_entity_update(&self, app: &Application, entity: &Entity) {
        todo!()
    }

    fn update(&self, app: &Application, entity: &Entity) {
        println!("called mesh.update");
    }
}
