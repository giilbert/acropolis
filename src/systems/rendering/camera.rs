use bevy_ecs::prelude::*;

use crate::{
    components::{
        rendering::{Camera, CurrentCamera},
        GlobalTransform,
    },
    resources::rendering::CurrentCameraMatrixResource,
};

pub fn camera_view_matrix_update_system(
    mut camera_matrix_resource: ResMut<CurrentCameraMatrixResource>,
    query: Query<
        &GlobalTransform,
        (With<GlobalTransform>, With<CurrentCamera>, With<Camera>),
    >,
) {
    for transform in &query {
        camera_matrix_resource.view_matrix = transform.matrix;
    }
}

// pub fn camera_projection_matrix_update_system(
//     mut camera_matrix_resource: ResMut<CameraMatrixResource>,
//     mut query: Query<&mut Camera, Changed<Camera>>,
// ) {
// }
