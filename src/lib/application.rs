use crate::{
    components::{
        rendering::{Camera, CurrentCamera, Mesh, Vertex},
        DefaultBundle, Name,
    },
    resources::rendering::StateResource,
    systems::{
        rendering::mesh_render_system, transform::transform_propagate_system,
    },
};

use super::{rendering::Material, window::Window};
use bevy_ecs::prelude::*;
use cgmath::Deg;

const SHADER: &str = r#"

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> test: f32;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position + vec3<f32>(test), 1.0);
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

impl Application {
    pub fn new() -> Application {
        let window = pollster::block_on(Window::new());
        let state = window.state.clone();

        let mut world = World::new();
        world.insert_non_send_resource(StateResource(window.state.clone()));

        let material = Material::new(&state, SHADER).unwrap();
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: [0.0, 0.5, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
            },
        ];
        const INDICES: &[u32] = &[0, 1, 2];

        let camera =
            Camera::new_perspective(&state.lock(), Deg(70.0), 0.1, 100.0);
        world.spawn_empty().insert(Mesh::new(
            &state,
            &material,
            VERTICES.to_vec(),
            INDICES.to_vec(),
        ));

        world.spawn_empty().insert(camera).insert(CurrentCamera);

        let mut runtime_schedule = Schedule::default();
        // runtime_schedule.add_stage(
        //     "update",
        //     SystemStage::parallel().with_system(transform_propagate_system),
        // );
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
