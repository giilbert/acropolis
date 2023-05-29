use acropolis_math::GlobalTransform;
use bevy_ecs::prelude::*;

use crate::{
    components::{Camera, CurrentCamera},
    StateResource,
};

pub fn camera_view_matrix_update_system(
    mut query: Query<&mut Camera, With<CurrentCamera>>,
    state_resource: Res<StateResource>,
) {
    if !state_resource.is_changed() {
        return;
    }

    let mut state = state_resource.lock();

    for mut camera in &mut query {
        camera.update_projection_matrix(&mut *state);
    }
}

// pub fn camera_projection_matrix_update_system(
//     mut camera_matrix_resource: ResMut<CameraMatrixResource>,
//     mut query: Query<&mut Camera, Changed<Camera>>,
// ) {
// }
