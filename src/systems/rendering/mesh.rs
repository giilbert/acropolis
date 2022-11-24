use bevy_ecs::prelude::*;
use wgpu::{util::DeviceExt, CommandEncoderDescriptor};

use crate::{
    components::{
        rendering::{Camera, CurrentCamera, Mesh},
        GlobalTransform,
    },
    resources::rendering::StateResource,
};

pub fn mesh_render_system(
    render_state: ResMut<StateResource>,
    mesh_query: Query<&Mesh>,
    camera_query: Query<&Camera, With<CurrentCamera>>,
) {
    let camera = camera_query.single();
    let state = &mut *render_state.lock();

    let view = &state.view.as_ref().unwrap();
    let encoder = state.encoder.as_mut().unwrap();
    let mut render_pass =
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Mesh Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations::default(),
            })],
            depth_stencil_attachment: None,
        });

    for mesh in &mesh_query {
        render_pass.set_pipeline(&mesh.render_pipeline);

        render_pass.set_bind_group(0, &camera.bind_group, &[]);

        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            mesh.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..(mesh.indices.len() as u32), 0, 0..1);
    }
}
