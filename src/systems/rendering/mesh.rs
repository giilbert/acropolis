use std::ops::DerefMut;

use bevy_ecs::prelude::*;
use wgpu::CommandEncoderDescriptor;

use crate::{
    components::{rendering::Mesh, GlobalTransform},
    resources::rendering::StateResource,
};

pub fn mesh_render_system(
    render_state: ResMut<StateResource>,
    query: Query<&Mesh>,
) {
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

    for mesh in &query {
        log::info!("Drawing {:?}", mesh);
        render_pass.set_pipeline(&mesh.render_pipeline);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            mesh.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..(mesh.indices.len() as u32), 0, 0..1);
    }
}
