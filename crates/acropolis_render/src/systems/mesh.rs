use acropolis_math::GlobalTransform;
use bevy_ecs::prelude::*;

use crate::{
    components::{Camera, CameraUniform, CurrentCamera, Mesh},
    resources::StateResource,
};

pub fn mesh_render_system(
    render_state: ResMut<StateResource>,
    mesh_query: Query<(&Mesh, &GlobalTransform)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CurrentCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let state = &mut *render_state.lock();

    let mut view_matrix: [[f32; 4]; 4] = camera_transform.matrix.into();
    // TODO: learn linalg and come back to this later?
    // i mean it works??
    view_matrix[3][0] *= -1.0;
    view_matrix[3][1] *= -1.0;
    let camera_uniform = CameraUniform {
        projection_matrix: camera.projection_matrix.into(),
        view_matrix,
    };
    state.queue.write_buffer(
        &camera.projection_matrix_buffer,
        0,
        bytemuck::cast_slice(&[camera_uniform]),
    );

    let view = &state.view.as_ref().unwrap();
    let encoder = state.encoder.as_mut().unwrap();
    let mut render_pass =
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Mesh Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.025,
                        g: 0.01,
                        b: 0.025,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

    render_pass.set_bind_group(0, &camera.bind_group, &[]);

    for (mesh, transform) in &mesh_query {
        render_pass.set_pipeline(&mesh.render_pipeline);

        render_pass.set_bind_group(1, &mesh.bind_group, &[]);
        let transformation_matrix: [[f32; 4]; 4] = transform.matrix.into();
        state.queue.write_buffer(
            &mesh.transformation_matrix_buffer,
            0,
            bytemuck::cast_slice(&transformation_matrix),
        );

        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            mesh.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..(mesh.indices.len() as u32), 0, 0..1);
    }
}
