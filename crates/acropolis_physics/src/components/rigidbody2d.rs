use acropolis_scripting::serde_json::Value;
use bevy_ecs::prelude::*;
use nalgebra::{Isometry2, Matrix4, Vector3};
use rapier2d::prelude::*;

use super::collider2d::Collider2D;
use crate::resources::{PhysicsInner, PhysicsResource};

#[derive(Component)]
pub struct RigidBody2D {
    pub rigidbody_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl RigidBody2D {
    pub fn load(
        physics_resource: PhysicsResource,
        value: Value,
        collider: &mut Collider2D,
    ) -> Self {
        let mut physics_resource = physics_resource.lock();

        let rigidbody = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 2.0])
            .build();

        let rigidbody_handle =
            physics_resource.rigid_body_set.insert(rigidbody);

        let collider_handle = collider
            .attach_rigidbody2d(&mut *physics_resource, rigidbody_handle);

        Self {
            rigidbody_handle,
            collider_handle,
        }
    }

    pub fn get_transformation(
        &self,
        physics_resource: &PhysicsInner,
    ) -> Matrix4<f32> {
        let rigidbody = physics_resource
            .rigid_body_set
            .get(self.rigidbody_handle)
            .unwrap();

        let translation = rigidbody.position().translation.vector;
        let rotation = rigidbody.position().rotation.angle();

        Matrix4::new_translation(&Vector3::new(
            translation.x,
            translation.y,
            0.0,
        )) * Matrix4::from_euler_angles(0.0, 0.0, rotation)
    }
}
