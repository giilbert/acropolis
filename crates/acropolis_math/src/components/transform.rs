use acropolis_scripting::{serde_json, Scriptable};
use nalgebra::{
    Isometry3, Matrix4, Quaternion, Translation3, UnitQuaternion, Vector3,
};
use serde::{Deserialize, Serialize};
use std::ops::Mul;

use bevy_ecs::{
    prelude::{Component, Entity},
    world::World,
};

#[derive(Component)]
pub struct Transform {
    // TODO: write macros for these bindings
    // 0
    pub position: Translation3<f32>,
    // 1
    pub rotation: UnitQuaternion<f32>,
    // 2
    pub scale: Vector3<f32>,
}

#[derive(Deserialize)]
struct TransformData {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
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
            position: Translation3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::identity(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn set_position(&mut self, translation: Translation3<f32>) {
        self.position = translation;
    }

    pub fn set_rotation(&mut self, rotation: UnitQuaternion<f32>) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
    }

    pub fn generate_matrix(&self) -> Matrix4<f32> {
        let matrix = Matrix4::new_translation(&self.position.vector)
            * Isometry3::from_parts(Translation3::identity(), self.rotation)
                .to_matrix()
            * Matrix4::new_nonuniform_scaling(&self.scale);

        return matrix;
    }

    pub fn generate_matrix_parent(
        &self,
        parent_matrix: &Matrix4<f32>,
    ) -> Matrix4<f32> {
        let local_matrix = self.generate_matrix();
        return local_matrix * parent_matrix;
    }

    pub fn from_json(
        _world: &mut World,
        value: acropolis_scripting::serde_json::Value,
    ) -> Self {
        let data: TransformData = serde_json::from_value(value).unwrap();
        let mut transform = Transform::new();
        transform.set_position(Translation3::from(data.position));
        transform.set_rotation(UnitQuaternion::from_quaternion(
            Quaternion::from(data.rotation),
        ));
        transform.set_scale(Vector3::from(data.scale));

        transform
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

// TODO: find a more permenant place
#[derive(Serialize, Deserialize)]
struct JsVector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Scriptable for Transform {
    fn set_property_vec3(&mut self, property: u32, x: f64, y: f64, z: f64) {
        match property {
            0 => {
                self.position = Translation3::new(x as f32, y as f32, z as f32)
            }
            2 => self.scale = Vector3::new(x as f32, y as f32, z as f32),
            _ => panic!("Invalid property"),
        }
    }

    fn get_property_vec3(&self, property: u32) -> (f64, f64, f64) {
        match property {
            0 => (
                self.position.x as f64,
                self.position.y as f64,
                self.position.z as f64,
            ),
            2 => (
                self.scale.x as f64,
                self.scale.y as f64,
                self.scale.z as f64,
            ),
            _ => panic!("Invalid property"),
        }
    }
}
