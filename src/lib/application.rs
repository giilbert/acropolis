use crate::{
    components::rendering::{Mesh, Vertex},
    resources::rendering::StateResource,
    systems::rendering::mesh_render_system,
};

use super::{rendering::Material, window::Window};
use bevy_ecs::prelude::*;

const SHADER: &str = r#"

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
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
        world.spawn().insert(Mesh::new(
            &state,
            &material,
            VERTICES.to_vec(),
            INDICES.to_vec(),
        ));

        let mut runtime_schedule = Schedule::default();
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
            let mut state_temp = state.lock();

            let frame = state_temp
                .surface
                .get_current_texture()
                .expect("Failed to acquire next swap chain texture");
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            state_temp.view = Some(view);
            state_temp.encoder =
                Some(state_temp.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {
                        label: Some("Command Encoder"),
                    },
                ));

            drop(state_temp);
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
