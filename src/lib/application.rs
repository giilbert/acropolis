use std::vec;

use crate::components::rendering::Mesh;
use crate::components::{
    Children, DefaultBundle, GlobalTransform, Name, Parent, Transform,
};
use crate::resources::core::Root;
use crate::resources::rendering::{GlResource, MaterialsResource};
use crate::systems::rendering::mesh_render_system;
use crate::systems::transform::transform_propagate_system;
use bevy_ecs::{
    prelude::World,
    schedule::{Schedule, SystemStage},
};

use super::rendering::{Material, Vertex};
use super::window::Window;
use glow::HasContext;

pub struct Application {
    window: Window,
    pub world: World,
    schedule: Schedule,
}

impl Application {
    pub fn new() -> Application {
        let window = Window::new();

        let mut world = World::default();
        world.insert_non_send_resource(GlResource(window.gl.clone()));

        let root_id = {
            let mut root = world.spawn();
            root.insert(Transform::new())
                .insert(GlobalTransform::new())
                .insert(Name("root".to_string()))
                .id()
        };
        let parent_id = {
            let mut parent = world.spawn();
            parent
                .insert_bundle(DefaultBundle {
                    name: Name("parent".to_string()),
                    transform: Transform::new(),
                    global_transform: GlobalTransform::new(),
                    children: Children(vec![]),
                    parent: Parent(root_id),
                })
                .insert(Mesh::new(window.gl.clone(), vec![], vec![]))
                .id()
        };
        let child_id = {
            let mut child = world.spawn();
            child
                .insert_bundle(DefaultBundle {
                    name: Name("child".to_string()),
                    transform: Transform::new(),
                    global_transform: GlobalTransform::new(),
                    children: Children(vec![]),
                    parent: Parent(parent_id),
                })
                .id()
        };
        let child_child_id = {
            let mut child = world.spawn();
            child
                .insert_bundle(DefaultBundle {
                    name: Name("child of child".to_string()),
                    transform: Transform::new(),
                    global_transform: GlobalTransform::new(),
                    children: Children(vec![]),
                    parent: Parent(child_id),
                })
                .insert(Mesh::new(
                    window.gl.clone(),
                    vec![
                        // generate vertex positions for a square
                        Vertex {
                            position: [1.0, 1.0, 0.0],
                        },
                        Vertex {
                            position: [-1.0, 1.0, 0.0],
                        },
                        Vertex {
                            position: [-1.0, -1.0, 0.0],
                        },
                        Vertex {
                            position: [1.0, -1.0, 0.0],
                        },
                    ],
                    // 1 0
                    // 2 3
                    vec![
                        // generate indices for a square
                        0, 1, 2, 0, 2, 3,
                    ],
                ))
                .id()
        };

        world
            .get_mut::<Children>(parent_id)
            .unwrap()
            .0
            .push(child_id);
        world
            .get_mut::<Children>(child_id)
            .unwrap()
            .0
            .push(child_child_id);

        world.insert_resource(Root::new(root_id));

        let mut material = match Material::new(
            window.gl.clone(),
            r#"#version 410
        in vec3 aVertexPosition;

        void main() {
            gl_Position = vec4(aVertexPosition, 1.0);
        }
        "#
            .to_string(),
            r#"#version 410
        precision mediump float;

        out vec4 fragColor;

        void main() {
            fragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }
        "#
            .to_string(),
            vec!["aVertexPosition"],
            Vec::<String>::new(),
        ) {
            Ok(material) => material,
            Err(err) => {
                log::error!("Material creation error\n{}", err);
                panic!();
            }
        };

        material.bind_mesh(&mut world, child_child_id);
        world.insert_non_send_resource(MaterialsResource(vec![material]));

        let mut schedule = Schedule::default();
        schedule.add_stage(
            "update",
            SystemStage::parallel()
                .with_system(transform_propagate_system)
                .with_system(mesh_render_system),
        );

        unsafe {
            let gl = window.gl.clone();
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.enable(glow::CULL_FACE);
        }

        Application {
            window,
            world,
            schedule,
        }
    }

    pub fn run(mut self) {
        let gl = self.window.gl.clone();

        let update = move || {
            // clear screen
            unsafe {
                gl.clear(glow::COLOR_BUFFER_BIT);
            }

            self.schedule.run_once(&mut self.world);
        };

        self.window.run_event_loop(Box::new(update));
    }
}
