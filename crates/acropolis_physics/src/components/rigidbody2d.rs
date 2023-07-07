use acropolis_scripting::{serde_json::Value, Scriptable};
use bevy_ecs::prelude::*;
use deno_core::serde_v8;
use nalgebra::{Matrix4, Vector2, Vector3};
use rapier2d::prelude::*;

use super::collider2d::Collider2D;
use crate::resources::{PhysicsInner, PhysicsResource};

#[derive(Component)]
pub struct RigidBody2D {
    pub rigidbody_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    physics_resource: PhysicsResource,
}

impl RigidBody2D {
    pub fn load(
        physics_resource: PhysicsResource,
        value: Value,
        collider: &mut Collider2D,
    ) -> Self {
        let mut physics_resource_inner = physics_resource.lock();

        let rigidbody = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 2.0])
            .build();

        let rigidbody_handle =
            physics_resource_inner.rigid_body_set.insert(rigidbody);

        let collider_handle = collider
            .attach_rigidbody2d(&mut *physics_resource_inner, rigidbody_handle);

        drop(physics_resource_inner);

        Self {
            rigidbody_handle,
            collider_handle,
            physics_resource,
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

impl Scriptable for RigidBody2D {
    fn call_component_method_mut(
        &mut self,
        method_id: u32,
        handle_scope: &mut deno_core::v8::HandleScope,
        arguments: deno_core::v8::Local<deno_core::v8::Value>,
    ) {
        let mut physics = self.physics_resource.lock();

        match method_id {
            0 => {
                let (f_x, f_y): (f32, f32) =
                    serde_v8::from_v8(handle_scope, arguments)
                        .expect("error deserializing");
                let rb = physics
                    .rigid_body_set
                    .get_mut(self.rigidbody_handle)
                    .expect("rigidbody handle not found.");
                rb.add_force(Vector2::new(f_x, f_y), true);
            }
            1 => {
                let (f_x, f_y): (f32, f32) =
                    serde_v8::from_v8(handle_scope, arguments)
                        .expect("error deserializing");
                let rb = physics
                    .rigid_body_set
                    .get_mut(self.rigidbody_handle)
                    .expect("rigidbody handle not found.");
                rb.apply_impulse(Vector2::new(f_x, f_y), true);
            }
            _ => panic!("Unknown method id"),
        }
    }
}
