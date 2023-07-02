use acropolis_scripting::serde_json::{self, Value};
use bevy_ecs::prelude::*;
use rapier2d::prelude::*;

use crate::resources::{PhysicsInner, PhysicsResource};

#[derive(Component)]
pub struct Collider2D {
    pub collider_handle: ColliderHandle,
    pub data: Collider2DData,
}

#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collider2DData {
    shape: ColliderShapeData,
    // TODO
}

#[derive(serde::Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ColliderShapeData {
    Rectangle {
        #[serde(rename = "halfExtents")]
        half_extents: [f32; 2],
    },
    // TODO
}

impl Into<ColliderShape> for ColliderShapeData {
    fn into(self) -> ColliderShape {
        match self {
            Self::Rectangle { half_extents } => {
                ColliderShape::cuboid(half_extents[0], half_extents[1])
            }
        }
    }
}

impl Collider2D {
    pub fn load(physics_resource: PhysicsResource, value: Value) -> Self {
        let mut physics_resource = physics_resource.lock();
        let data =
            serde_json::from_value::<Collider2DData>(value.clone()).unwrap();

        // TODO: add the rest of the properties
        let collider = ColliderBuilder::new(data.shape.clone().into()).build();
        let collider_handle = physics_resource.collider_set.insert(collider);

        Self {
            collider_handle,
            data,
        }
    }

    pub fn attach_rigidbody2d(
        &mut self,
        physics_resource: &mut PhysicsInner,
        rigidbody_handle: RigidBodyHandle,
    ) -> ColliderHandle {
        let PhysicsInner {
            ref mut rigid_body_set,
            ref mut collider_set,
            ref mut island_manager,
            ..
        } = *physics_resource;

        let collider =
            ColliderBuilder::new(self.data.shape.clone().into()).build();

        // remove the old collider
        collider_set.remove(
            self.collider_handle,
            island_manager,
            rigid_body_set,
            true,
        );

        // insert the new collider with the rigidbody
        self.collider_handle = collider_set.insert_with_parent(
            collider,
            rigidbody_handle,
            rigid_body_set,
        );

        self.collider_handle.clone()
    }
}
