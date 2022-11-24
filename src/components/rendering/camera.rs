use std::sync::Arc;

use bevy_ecs::prelude::Component;
use cgmath::{Matrix4, Rad};
use wgpu::{util::DeviceExt, BindGroup, Buffer};

use crate::lib::{
    rendering::{State, StateInner},
    window::WINDOW_SIZE,
};

#[derive(Component)]
pub struct Camera {
    pub matrix: Matrix4<f32>,
    pub camera_data: CameraData,
    pub bind_group: BindGroup,
    pub matrix_buffer: Buffer,
}

#[derive(Component)]
pub struct CurrentCamera;

pub enum CameraData {
    Perspective {
        fov: Rad<f32>,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    },
    Orthographic {},
}

impl Camera {
    pub fn new_perspective<T>(
        state: &StateInner,
        fov: T,
        near: f32,
        far: f32,
    ) -> Camera
    where
        T: Copy + Into<Rad<f32>>,
    {
        // create perspective matrix from fov
        let (x, y) = *WINDOW_SIZE.read().unwrap();
        let aspect_ratio = x / y;
        let matrix = cgmath::perspective(fov, aspect_ratio.into(), near, far);

        let matrix_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Uniform Buffer"),
                contents: bytemuck::cast_slice(&[0.7f32]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            },
        );

        let bind_group_layout = state.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Mesh Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            },
        );

        let bind_group =
            state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Mesh Bind Group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: matrix_buffer.as_entire_binding(),
                }],
            });

        Camera {
            matrix,
            camera_data: CameraData::Perspective {
                fov: fov.into(),
                aspect_ratio,
                near,
                far,
            },
            bind_group,
            matrix_buffer,
        }
    }
}
