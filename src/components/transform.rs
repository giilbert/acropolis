use std::ops::Mul;

use crate::{lib::scripting::scripting_api::ScriptingApi, utils::types::*};
use bevy_ecs::prelude::{Component, Entity};
use cgmath::{SquareMatrix, Zero};
use deno_core::{serde_json, serde_v8::Value};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

#[derive(Component)]
pub struct GlobalTransform {
    pub matrix: Matrix4,
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

    pub fn set_position(&mut self, translation: Vector3) {
        self.position = translation;
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
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::new()
    }
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

impl Default for GlobalTransform {
    fn default() -> GlobalTransform {
        GlobalTransform::new()
    }
}

#[derive(Serialize, Deserialize)]
struct JsVector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ScriptingApi for Transform {
    fn set_property(&mut self, name: &str, value: String) {
        match name {
            "position" => {
                let JsVector3 { x, y, z } =
                    serde_json::from_str(&value).unwrap();
                self.position.x = x;
                self.position.y = y;
                self.position.z = z;
            }
            "scale" => {
                let JsVector3 { x, y, z } =
                    serde_json::from_str(&value).unwrap();
                self.scale.x = x;
                self.scale.y = y;
                self.scale.z = z;
            }
            _ => panic!("bad property"),
        }
    }

    fn get_property(&self, name: &str) -> String {
        match name {
            "position" => {
                let payload = JsVector3 {
                    x: self.position.x,
                    y: self.position.y,
                    z: self.position.z,
                };
                serde_json::to_string(&payload).unwrap()
            }
            "scale" => {
                let payload = JsVector3 {
                    x: self.scale.x,
                    y: self.scale.y,
                    z: self.scale.z,
                };
                serde_json::to_string(&payload).unwrap()
            }
            _ => panic!("bad property"),
        }
    }
}
