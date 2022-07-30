use std::ops::Mul;

use crate::utils::types::*;
use bevy_ecs::prelude::{Component, Entity};
use cgmath::{SquareMatrix, Zero};

#[derive(Component)]
pub struct Transform {
    position: Vector3,
    rotation: Quaternion,
    scale: Vector3,
}

#[derive(Component)]
pub struct GlobalTransform {
    pub matrix: Matrix4,
}

#[derive(Component)]
pub struct Parent(pub Entity);

#[derive(Component, Clone)]
pub struct Children(pub Vec<Entity>);

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn generate_matrix(&self) -> Matrix4 {
        let matrix = Matrix4::from_translation(self.position)
            .mul(Matrix4::from_nonuniform_scale(
                self.scale.x,
                self.scale.y,
                self.scale.z,
            ))
            .mul(Matrix4::from(self.rotation));

        return matrix;
    }

    pub fn generate_matrix_parent(&self, parent_matrix: &Matrix4) -> Matrix4 {
        let local_matrix = self.generate_matrix();
        return local_matrix * parent_matrix;
    }

    //     pub fn update_matrix(&mut self) {
    //         let matrix = self.generate_matrix();
    //         self.matrix = matrix;
    //      }

    pub fn set_position(&mut self, position: Vector3) {
        self.position = position;
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }

    //     pub fn set_rotation(&mut self, rotation: Quaternion) {
    //         self.rotation = rotation;
    //     }

    //     pub fn set_scale(&mut self, scale: Vector3) {
    //         self.scale = scale;
    //     }
}

impl GlobalTransform {
    pub fn new() -> GlobalTransform {
        GlobalTransform {
            matrix: Matrix4::identity(),
        }
    }

    pub fn generate_matrix_parent(
        &self,
        local_matrix: &Matrix4,
        parent_matrix: &Matrix4,
    ) -> Matrix4 {
        return local_matrix * parent_matrix;
    }
}
