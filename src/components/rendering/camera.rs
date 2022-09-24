use bevy_ecs::prelude::Component;
use cgmath::{Matrix4, Rad};

use crate::lib::window::WINDOW_SIZE;

#[derive(Component)]
pub struct Camera {
    pub matrix: Matrix4<f32>,
    pub camera_data: CameraData,
}

#[derive(Component)]
pub struct CurrentCameraMarker();

pub enum CameraData {
    Perspective(PerspectiveCameraData),
    Orthographic(OrthographicCameraData),
}

struct PerspectiveCameraData {
    fov: Rad<f32>,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}
struct OrthographicCameraData {}

impl Camera {
    pub fn new_perspective<T>(fov: T, near: f32, far: f32) -> Camera
    where
        T: Copy + Into<Rad<f32>>,
    {
        // create perspective matrix from fov
        let (x, y) = *WINDOW_SIZE.read().unwrap();
        let aspect_ratio = x / y;
        let matrix = cgmath::perspective(fov, aspect_ratio.into(), near, far);

        Camera {
            matrix,
            camera_data: CameraData::Perspective(PerspectiveCameraData {
                fov: fov.into(),
                aspect_ratio,
                near,
                far,
            }),
        }
    }
}
