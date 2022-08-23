use bevy_ecs::prelude::Component;
use cgmath::Rad;

use crate::{
    lib::window::WINDOW_SIZE,
    utils::types::{Matrix4, Precision},
};

#[derive(Component)]
pub struct Camera {
    pub matrix: Matrix4,
    pub camera_data: CameraData,
}

#[derive(Component)]
pub struct CurrentCameraMarker();

pub enum CameraData {
    Perspective(PerspectiveCameraData),
    Orthographic(OrthographicCameraData),
}

struct PerspectiveCameraData {
    fov: Rad<Precision>,
    aspect_ratio: Precision,
    near: Precision,
    far: Precision,
}
struct OrthographicCameraData {}

impl Camera {
    pub fn new_perspective<T>(fov: T, near: Precision, far: Precision) -> Camera
    where
        T: Copy + Into<Rad<Precision>>,
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
