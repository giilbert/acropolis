use std::time::Instant;

use crate::components::rendering::{Camera, CurrentCameraMarker, Mesh};
use crate::components::{
    Children, DefaultBundle, GlobalTransform, Name, Parent, Transform,
};
use crate::resources::core::Root;
use crate::resources::rendering::{
    CurrentCameraMatrixResource, GlResource, MaterialsResource,
};
use crate::systems::rendering::{
    camera_view_matrix_update_system, mesh_render_system,
};
use crate::systems::transform::transform_propagate_system;
use bevy_ecs::query::With;
use bevy_ecs::system::Query;
use bevy_ecs::{
    prelude::World,
    schedule::{Schedule, SystemStage},
};
use cgmath::Deg;

use super::rendering::{Material, Vertex};
use super::window::Window;
use glow::HasContext;

fn test_system(mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut query {
        transform.position.z = -4.0;
        transform.position.x -= 0.01;
    }
}

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
                .insert(Children(vec![]))
                .id()
        };
        let camera_id = {
            let mut child = world.spawn();
            child
                .insert_bundle(DefaultBundle {
                    name: Name("camera".to_string()),
                    transform: Transform::new(),
                    global_transform: GlobalTransform::new(),
                    children: Children(vec![]),
                    parent: Parent(root_id),
                })
                .insert(Camera::new_perspective(Deg(60.0), 0.1, 100.0))
                .insert(CurrentCameraMarker {})
                .id()
        };
        let mesh_id = {
            let mut child = world.spawn();
            child
                .insert_bundle(DefaultBundle {
                    name: Name("mesh".to_string()),
                    transform: Transform::new(),
                    global_transform: GlobalTransform::new(),
                    children: Children(vec![]),
                    parent: Parent(root_id),
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
            .get_mut::<Children>(root_id)
            .unwrap()
            .0
            .push(camera_id);
        world.get_mut::<Children>(root_id).unwrap().0.push(mesh_id);

        // make the camera current
        let projection_matrix = world.get::<Camera>(camera_id).unwrap().matrix;
        let view_matrix =
            world.get::<GlobalTransform>(camera_id).unwrap().matrix;

        world.insert_resource(CurrentCameraMatrixResource {
            projection_matrix,
            view_matrix,
        });
        world.insert_resource(Root::new(root_id));

        let mut material = match Material::new(
            window.gl.clone(),
            r#"#version 410
        in vec3 aVertexPosition;

        uniform mat4 uModelMatrix;
        uniform mat4 uViewMatrix;
        uniform mat4 uProjectionMatrix;

        void main() {
            uModelMatrix;
            gl_Position = uProjectionMatrix * uViewMatrix * mat4(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ) * vec4(aVertexPosition, 1.0);
        }
        "#
            .to_string(),
            r#"#version 410
        precision mediump float;

        out vec4 fragColor;

        void main() {
            fragColor = vec4(0.0, 1.0, 0.0, 1.0);
        }
        "#
            .to_string(),
            vec!["aVertexPosition"],
            vec!["uProjectionMatrix", "uViewMatrix", "uModelMatrix"],
        ) {
            Ok(material) => material,
            Err(err) => {
                log::error!("Material creation error\n{}", err);
                panic!();
            }
        };

        material.bind_mesh(&mut world, mesh_id);
        world.insert_non_send_resource(MaterialsResource(vec![material]));

        let mut schedule = Schedule::default();
        schedule.add_stage(
            "update",
            SystemStage::parallel()
                .with_system(test_system)
                .with_system(transform_propagate_system)
                .with_system(mesh_render_system)
                .with_system(camera_view_matrix_update_system),
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
        let mut then = Instant::now();

        let update = move || {
            // clear screen
            unsafe {
                gl.clear(glow::COLOR_BUFFER_BIT);
            }

            self.schedule.run_once(&mut self.world);

            let dt = Instant::now().duration_since(then).as_millis();
            then = Instant::now();

            log::info!("dt: {}ms | fps: {}", dt, 1000.0 / dt as f32);
        };

        self.window.run_event_loop(Box::new(update));
    }
}
