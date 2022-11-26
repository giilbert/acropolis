use crate::{
    components::{
        rendering::{Camera, CurrentCamera, Mesh, Vertex},
        Children, DefaultBundle, GlobalTransform, Name, Parent, Transform,
    },
    resources::{core::Root, rendering::StateResource},
    systems::{
        rendering::mesh_render_system, transform::transform_propagate_system,
    },
};

use super::{rendering::Material, window::Window};
use bevy_ecs::prelude::*;
use cgmath::{Deg, Vector3};

const SHADER: &str = r#"

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct CameraMatrices {   
    projection_matrix: mat4x4<f32>,
    view_matrix: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> camera_matrices: CameraMatrices;

@group(1) @binding(0)
var<uniform> model_matrix: mat4x4<f32>;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position =
        model_matrix
        * camera_matrices.projection_matrix
        * camera_matrices.view_matrix
        * vec4<f32>(model.position, 1.0);

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.1, 0.5, 1.0);
}

"#;

pub struct Application {
    window: Window,
    pub world: World,
    runtime_schedule: Schedule,
}

fn test_system(mut query: Query<&mut Transform, With<Mesh>>) {
    let mut transform = query.single_mut();
    transform.position.x += 0.01;
}

impl Application {
    pub fn new() -> Application {
        let window = pollster::block_on(Window::new());
        let state = window.state.clone();

        let mut world = World::new();
        world.insert_non_send_resource(StateResource(window.state.clone()));

        let material = Material::new(&state, SHADER).unwrap();
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: [0.5, 0.5, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
            },
        ];
        const INDICES: &[u32] = &[0, 2, 1, 0, 3, 2];

        let root = world
            .spawn_empty()
            .insert(Name("Root".into()))
            .insert(Transform::new())
            .insert(GlobalTransform::new())
            .id();

        world.insert_resource(Root(root.clone()));

        world
            .spawn_empty()
            .insert(Name("Mesh".into()))
            .insert(Parent(root))
            .insert(Children(vec![]))
            .insert(Transform::new())
            .insert(GlobalTransform::new())
            .insert(Mesh::new(
                &state,
                &material,
                VERTICES.to_vec(),
                INDICES.to_vec(),
            ));

        let mut transform = Transform::new();
        transform.set_position(Vector3::new(0.0, 0.0, -4.0));
        world
            .spawn_empty()
            .insert(DefaultBundle {
                transform,
                name: Name("Camera".into()),
                parent: Parent(root.clone()),
                children: Children(vec![]),
                global_transform: GlobalTransform::new(),
            })
            .insert(Camera::new_perspective(
                &state.lock(),
                Deg(50.0),
                0.1,
                1000.0,
            ))
            .insert(CurrentCamera);

        let mut runtime_schedule = Schedule::default();
        runtime_schedule.add_stage(
            "scripting",
            SystemStage::parallel().with_system(test_system),
        );
        runtime_schedule.add_stage(
            "update",
            SystemStage::parallel().with_system(transform_propagate_system),
        );
        runtime_schedule.add_stage(
            "render",
            SystemStage::parallel().with_system(mesh_render_system),
        );

        Application {
            window,
            world,
            runtime_schedule,
        }
    }

    pub fn run(mut self) {
        let state = self.window.state.clone();

        self.window.run_event_loop(state.clone(), move || {
            let frame = {
                let mut state = state.lock();

                let frame = state
                    .surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                state.view = Some(view);
                state.encoder = Some(state.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {
                        label: Some("Command Encoder"),
                    },
                ));

                frame
            };

            self.runtime_schedule.run(&mut self.world);

            {
                let mut state = state.lock();
                let commands = state.encoder.take().unwrap().finish();
                state.queue.submit(Some(commands));
                frame.present();
            }
        });
    }
}
