use bevy_ecs::prelude::Component;
use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Rad};
use wgpu::{util::DeviceExt, BindGroup, Buffer};

use crate::lib::{rendering::StateInner, window::WINDOW_SIZE};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Component)]
pub struct Camera {
    pub projection_matrix: Matrix4<f32>,
    pub camera_data: CameraData,
    pub bind_group: BindGroup,
    pub projection_matrix_buffer: Buffer,
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
        let projection_matrix = OPENGL_TO_WGPU_MATRIX
            * cgmath::perspective(fov, aspect_ratio.into(), near, far);

        let projection_matrix_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Uniform Buffer"),
                #[rustfmt::skip]
                contents: bytemuck::cast_slice(&[0.0f32; 64]),
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
                label: Some("Camera Bind Group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection_matrix_buffer.as_entire_binding(),
                }],
            });

        Camera {
            projection_matrix,
            camera_data: CameraData::Perspective {
                fov: fov.into(),
                aspect_ratio,
                near,
                far,
            },
            bind_group,
            projection_matrix_buffer,
        }
    }
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct CameraUniform {
    pub projection_matrix: [[f32; 4]; 4],
    pub view_matrix: [[f32; 4]; 4],
}
