use std::ops::Mul;

use bevy_ecs::prelude::{Component, Entity};
use cgmath::{Matrix4, Quaternion, SquareMatrix, Vector3, Zero};

#[derive(Component)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

#[derive(Component)]
pub struct GlobalTransform {
    pub matrix: Matrix4<f32>,
}

#[derive(Component)]
pub struct Parent(pub Entity);

#[derive(Component)]
pub struct Children(pub Vec<Entity>);

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn set_position(&mut self, translation: Vector3<f32>) {
        self.position = translation;
    }

    pub fn generate_matrix(&self) -> Matrix4<f32> {
        let matrix = Matrix4::from_translation(self.position)
            .mul(Matrix4::from_nonuniform_scale(
                self.scale.x,
                self.scale.y,
                self.scale.z,
            ))
            .mul(Matrix4::from(self.rotation));

        return matrix;
    }

    pub fn generate_matrix_parent(
        &self,
        parent_matrix: &Matrix4<f32>,
    ) -> Matrix4<f32> {
        let local_matrix = self.generate_matrix();
        return local_matrix * parent_matrix;
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::new()
    }
}

impl GlobalTransform {
    pub fn new() -> GlobalTransform {
        GlobalTransform {
            matrix: Matrix4::<f32>::identity(),
        }
    }

    pub fn generate_matrix_parent(
        &self,
        local_matrix: &Matrix4<f32>,
        parent_matrix: &Matrix4<f32>,
    ) -> Matrix4<f32> {
        return local_matrix * parent_matrix;
    }
}

impl Default for GlobalTransform {
    fn default() -> GlobalTransform {
        GlobalTransform::new()
    }
}
