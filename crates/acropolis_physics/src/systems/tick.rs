use acropolis_math::{GlobalTransform, Parent, Transform};
use bevy_ecs::prelude::*;

use crate::{
    components::{collider2d::Collider2D, rigidbody2d::RigidBody2D},
    resources::{PhysicsInner, PhysicsResource},
};

pub fn tick_system(physics: ResMut<PhysicsResource>) {
    let PhysicsInner {
        ref mut pipeline,
        ref gravity,
        ref integration_parameters,
        ref mut island_manager,
        ref mut broad_phase,
        ref mut narrow_phase,
        ref mut rigid_body_set,
        ref mut collider_set,
        ref mut impulse_joint_set,
        ref mut multibody_joint_set,
        ref mut ccd_solver,
        ..
    } = *physics.lock();

    pipeline.step(
        &gravity,
        &integration_parameters,
        island_manager,
        broad_phase,
        narrow_phase,
        rigid_body_set,
        collider_set,
        impulse_joint_set,
        multibody_joint_set,
        ccd_solver,
        None,
        &(), // physics hooks
        &(), // event handler
    );
}

pub fn update_positions_post_tick(
    rigidbody2ds: Query<(Entity, &RigidBody2D, &Parent)>,
    mut transform_query: Query<(&mut GlobalTransform, &mut Transform)>,
    physics: ResMut<PhysicsResource>,
) {
    let physics = physics.lock();

    for (entity, rigidbody2d, parent) in &rigidbody2ds {
        let parent_global_matrix = transform_query
            .get(parent.0)
            .expect("RigidBody2D should have a transform")
            .0
            .matrix;

        let (mut global_transform, mut local_transform) = transform_query
            .get_mut(entity)
            .expect("RigidBody2D should have a transform");

        let transformation = rigidbody2d.get_transformation(&*physics);

        global_transform.update_from_rigidbody_global_transform(transformation);
        local_transform.update_from_rigidbody_global_transform(
            &global_transform.matrix,
            &parent_global_matrix,
        );
    }
}

pub fn update_positions_pre_tick(
    mut rigidbody2ds: Query<(&GlobalTransform, &RigidBody2D)>,
    mut colliders2ds: Query<(&GlobalTransform, &Collider2D)>,
    physics: ResMut<PhysicsResource>,
) {
    let mut physics = physics.lock();

    for (global_transform, rigidbody2d) in &mut rigidbody2ds {
        let isometry = global_transform.as_isometry2();
        let rigidbody = physics
            .rigid_body_set
            .get_mut(rigidbody2d.rigidbody_handle)
            .unwrap();

        rigidbody.set_position(isometry, true);
        rigidbody.set_rotation(isometry.rotation, true);
    }

    for (global_transform, collider2d) in &mut colliders2ds {
        let isometry = global_transform.as_isometry2();
        let collider = physics
            .collider_set
            .get_mut(collider2d.collider_handle)
            .unwrap();
        collider.set_position(isometry);
        collider.set_rotation(isometry.rotation);
    }
}
