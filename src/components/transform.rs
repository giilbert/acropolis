use crate::utils::types::*;
use bevy_ecs::prelude::{Component, Entity};
use cgmath::Zero;

#[derive(Component)]
pub struct Transform {
    position: Vector3,
    quaternion: Quaternion,
    scale: Vector3,
    children: Vec<Entity>,
    pub matrix: Matrix4,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3::zero(),
            quaternion: Quaternion::zero(),
            scale: Vector3::zero(),
            children: vec![],
            matrix: Matrix4::zero(),
        }
    }
}
