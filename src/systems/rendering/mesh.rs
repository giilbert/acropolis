use core::slice;

use bevy_ecs::prelude::*;
use cgmath::{Matrix, SquareMatrix};
use glow::HasContext;

use crate::{
    components::{rendering::Mesh, GlobalTransform, Parent},
    resources::rendering::{
        CurrentCameraMatrixResource, GlResource, MaterialsResource,
    },
};

pub fn mesh_render_system(
    gl: NonSend<GlResource>,
    camera_matrices: Res<CurrentCameraMatrixResource>,
    materials: NonSend<MaterialsResource>,
    query: Query<(&Mesh, &GlobalTransform), With<Parent>>,
) {
    for material in &materials.0 {
        material.bind();

        // set the uProjectionMatrix uniform
        // to the projection matrix of the camera
        let projection_matrix = camera_matrices.projection_matrix;
        let view_matrix = camera_matrices.view_matrix;
        // let projection_matrix = Matrix4::identity();
        // let view_matrix = Matrix4::identity();
        unsafe {
            gl.uniform_matrix_4_f32_slice(
                Some(&material.uniforms[0].location),
                false,
                slice::from_raw_parts(projection_matrix.as_ptr(), 16),
            );

            gl.uniform_matrix_4_f32_slice(
                Some(&material.uniforms[1].location),
                false,
                slice::from_raw_parts(view_matrix.as_ptr(), 16),
            );
        }

        for entity in &material.entities {
            let mesh = query.get_component::<Mesh>(*entity).unwrap();
            let transform =
                query.get_component::<GlobalTransform>(*entity).unwrap();

            unsafe {
                gl.uniform_matrix_4_f32_slice(
                    Some(&material.uniforms[2].location),
                    false,
                    slice::from_raw_parts(transform.matrix.as_ptr(), 16),
                );
            }

            mesh.draw(&gl);
        }
    }
}
